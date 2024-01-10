use log::info;

use wgpu;
use winit::window::Window;

struct Context{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,

    particle_buffer: Vec<wgpu::Buffer>,
    vertex_buffer: wgpu::Buffer,

    // render_pipeline: wgpu::RenderPipeline,
    compute_pipeline: wgpu::ComputePipeline,
}

impl Context{
    pub async fn init(
        num_particles: u32,
        window: Window
    ) -> Self{

        let size = window.inner_size();

        info!("Window size: {}x{}", size.width, size.height);

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        info!("Created instance");

        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window, so this should be safe.

        let surface = unsafe { 
            instance.create_surface(&window).expect("Failed to create surface")
        };

        info!("Created surface");

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        info!("Created adapter");

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web, we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        info!("Created device and queue");

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result in all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        info!("Configured surface");

        // Initialize particle buffers

        // Initialize compute shader

        let compute_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("compute.wgsl").into()),
        });

        // Initialize compute bind group

        let compute_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new((num_particles * 24) as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new((num_particles * 24) as _),
                    },
                    count: None,
                },
            ],
            label: None,
        });

        let compute_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("compute"),
            bind_group_layouts: &[&compute_bind_group_layout],
            push_constant_ranges: &[],
        });

        // Initialize compute pipeline

        let compute_pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute pipeline"),
            layout: Some(&compute_pipeline_layout),
            module: &compute_shader,
            entry_point: "step_main",
        });

        // Initialize render shader

        // Initialize render bind group

        // Initialize render pipeline



        Context{
            surface,
            device,
            queue,

            particle_buffer: Vec::new(),
            vertex_buffer: wgpu::Buffer,

            // render_pipeline: wgpu::RenderPipeline,
            compute_pipeline,
        }
    }

    fn update(){
        // empty
    }

    fn resize(){
        // empty
    }

    fn render(&mut self){

        // !!!! Need to change render pass descriptor's relation with texture view

        // let render_pass_descriptor = wgpu::RenderPassDescriptor {
        //     label: None,
        //     color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
        //         attachment: &self.frame_texture_view,
        //         resolve_target: None,
        //         ops: wgpu::Operations {
        //             load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
        //             store: true,
        //         },
        //     }],
        //     depth_stencil_attachment: None,
        // };

        let mut command_enconder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            // compute pass
            let mut compute_pass = command_enconder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            // compute_pass.set_bind_group(0, &self.particle_bind_groups[self.frame_num % 2], &[]);
            // compute_pass.dispatch_workgroups(self.work_group_count, 1, 1);
        }
        self.queue.submit(Some(command_enconder.finish()));

        // {
        //     // render pass
        //     let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
        //     render_pass.set_pipeline(&self.render_pipeline);


        // }
    }
}