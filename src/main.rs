use std::time::{Instant, Duration};
use std::thread;
use std::sync::{Arc, Mutex};
use breakout::{BoardSize, Ball};

fn main() {
    use glium::{glutin, Surface};
    use glium::glutin::dpi::*;

    let boardsize = BoardSize {
        width: 400,
        height: 600,
    };

    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(LogicalSize::new(boardsize.width as f64, boardsize.height as f64))
        .with_title("Breakout");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    #[derive(Debug, Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    glium::implement_vertex!(Vertex, position);

    let vertex1 = Vertex { position: [ 0.0, 0.0] };
    let shape = vec![vertex1];

    let vertex_buffer = glium::VertexBuffer::dynamic(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::Points);

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        uniform vec2 boardsize;

        void main() {
            vec2 coord = position/boardsize*2.0-vec2(1.0);
            gl_Position = vec4(coord, 0.0, 1.0);
            gl_PointSize = 10.0;
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program =
        glium::Program::new(&display,
                            glium::program::ProgramCreationInput::SourceCode {
                                vertex_shader: vertex_shader_src,
                                fragment_shader: fragment_shader_src,
                                geometry_shader: None,
                                tessellation_control_shader: None,
                                tessellation_evaluation_shader: None,
                                transform_feedback_varyings: None,
                                outputs_srgb: false,
                                uses_point_size: true,
                            }).unwrap();

    let ball_position = Arc::new(Mutex::new([0.0f32, 0.0]));

    let child = {
        let boardsize = boardsize.clone();
        let ball_position = ball_position.clone();

        thread::spawn(move || {
            let mut ball = Ball {
                position: [0.0, 0.0],
                direction: [100.0, 141.42],
                radius: 2.0,
            };
            let mut time = Instant::now();

            loop {
                thread::sleep(Duration::from_millis(10));

                let curr_time = Instant::now();
                let step = (curr_time.duration_since(time).as_millis() as f32) / 1000.0;
                time = curr_time;

                ball.update(step, &boardsize);

                *ball_position.lock().unwrap() = ball.position;
            }
        })
    };

    let mut closed = false;
    while !closed {
        {
            let position = ball_position.lock().unwrap();
            vertex_buffer.write(&[Vertex { position: position.clone() }]);
        }

        let uniforms = glium::uniform! {
            boardsize: boardsize.clone()
        };

        let mut target = display.draw();
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        events_loop.poll_events(|ev| {
            match ev {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });
    }
}
