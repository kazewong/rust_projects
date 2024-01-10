use wgpu;

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
    fn init(

        num_particles: u32,
    ) -> Self{

        // Initialize particle buffers

        // Initialize compute shader

        let compute_shader = wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("compute.wgsl")),
        };

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
                        min_binding_size: wgpu::BufferSize::new((num_particles * 16) as _),
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: wgpu::BufferSize::new((num_particles * 16) as _),
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
            surface: surface,
            device: device,
            queue: queue,

            particle_buffer: Vec::new(),
            vertex_buffer: wgpu::Buffer,

            // render_pipeline: wgpu::RenderPipeline,
            compute_pipeline: wgpu::ComputePipeline,
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
            let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
                timestamp_writes: None,
            });
            compute_pass.set_pipeline(&self.compute_pipeline);
            // compute_pass.set_bind_group(0, &self.particle_bind_groups[self.frame_num % 2], &[]);
            // compute_pass.dispatch_workgroups(self.work_group_count, 1, 1);
        }

        // {
        //     // render pass
        //     let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);
        //     render_pass.set_pipeline(&self.render_pipeline);


        // }
    }
}