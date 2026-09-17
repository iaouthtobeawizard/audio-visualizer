use anyhow::Result;
use pipewire as pw;
use pw::{properties::properties, spa};
use spa::pod::Pod;
use std::sync::mpsc::{self, Receiver, Sender};

pub struct PipeWireCapture {
    receiver: Receiver<Vec<f32>>,
    _thread: std::thread::JoinHandle<()>,
}

struct UserData {
    sender: Sender<Vec<f32>>,
    format: spa::param::audio::AudioInfoRaw,
}

impl PipeWireCapture {
    pub fn new() -> Result<Self> {
        let (sender, receiver) = mpsc::channel();

        let thread = std::thread::spawn(move || {
            let _ = run_pipewire(sender);
        });

        Ok(Self {
            receiver,
            _thread: thread,
        })
    }

    pub fn recv(&self) -> Result<Vec<f32>> {
        self.receiver
            .recv()
            .map_err(|error| anyhow::anyhow!("PipeWire capture stopped: {error}"))
    }
}

fn run_pipewire(sender: Sender<Vec<f32>>) -> Result<()> {
    pw::init();

    let mainloop = pw::main_loop::MainLoopRc::new(None)?;

    let context = pw::context::ContextRc::new(&mainloop, None)?;

    let core = context.connect_rc(None)?;

    let props = properties! {
        *pw::keys::MEDIA_TYPE => "Audio",
        *pw::keys::MEDIA_CATEGORY => "Capture",
        *pw::keys::MEDIA_ROLE => "Music",
        *pw::keys::STREAM_CAPTURE_SINK => "true",
    };

    let stream = pw::stream::StreamBox::new(&core, "audio-visualizer", props)?;

    let data = UserData {
        sender,
        format: Default::default(),
    };

    let _listener = stream
        .add_local_listener_with_user_data(data)
        .param_changed(|_, user_data, id, param| {
            let Some(param) = param else {
                return;
            };

            if id != pw::spa::param::ParamType::Format.as_raw() {
                return;
            }

            let _ = user_data.format.parse(param);
        })
        .process(|stream, user_data| {
            let Some(mut buffer) = stream.dequeue_buffer() else {
                return;
            };

            let datas = buffer.datas_mut();

            if datas.is_empty() {
                return;
            }

            let data = &mut datas[0];

            let Some(bytes) = data.data() else {
                return;
            };

            let (chunks, _) = bytes.as_chunks::<4>();

            let samples: Vec<f32> = chunks
                .iter()
                .map(|chunk| f32::from_le_bytes(*chunk))
                .collect();

            let _ = user_data.sender.send(samples);
        })
        .register()?;

    let mut audio_info = spa::param::audio::AudioInfoRaw::new();

    audio_info.set_format(spa::param::audio::AudioFormat::F32LE);

    let obj = pw::spa::pod::Object {
        type_: pw::spa::utils::SpaTypes::ObjectParamFormat.as_raw(),
        id: pw::spa::param::ParamType::EnumFormat.as_raw(),
        properties: audio_info.into(),
    };

    let values = pw::spa::pod::serialize::PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &pw::spa::pod::Value::Object(obj),
    )
    .unwrap()
    .0
    .into_inner();

    let mut params = [Pod::from_bytes(&values).unwrap()];

    stream.connect(
        spa::utils::Direction::Input,
        None,
        pw::stream::StreamFlags::AUTOCONNECT
            | pw::stream::StreamFlags::MAP_BUFFERS
            | pw::stream::StreamFlags::RT_PROCESS,
        &mut params,
    )?;

    mainloop.run();

    Ok(())
}
