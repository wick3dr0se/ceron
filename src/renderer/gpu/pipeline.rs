use wgpu::*;

use crate::vertex::{INDICES, VERTICES, Vertex};

use super::{context::RenderContext, shader::Shader};

pub(crate) struct Pipeline;

impl Pipeline {
    pub(crate) fn new(ctx: &RenderContext) -> (RenderPipeline, Buffer, Buffer) {
        let vertex_buffer = util::DeviceExt::create_buffer_init(
            &ctx.device,
            &util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: BufferUsages::VERTEX,
            },
        );
        let index_buffer = util::DeviceExt::create_buffer_init(
            &ctx.device,
            &util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: BufferUsages::INDEX,
            },
        );

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: Some("Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });
        let pipeline = ctx
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&pipeline_layout),
                vertex: VertexState {
                    module: &Shader::load(&ctx, "vertex.wgsl"),
                    entry_point: Some("main"),
                    buffers: &[VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
                        step_mode: VertexStepMode::Vertex,
                        attributes: &Vertex::ATTRIBUTES,
                    }],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                cache: None,
                fragment: Some(FragmentState {
                    module: &Shader::load(&ctx, "fragment.wgsl"),
                    entry_point: Some("main"),
                    targets: &[Some(ColorTargetState {
                        format: ctx.config.format,
                        blend: Some(BlendState::REPLACE),
                        write_mask: ColorWrites::ALL,
                    })],
                    compilation_options: PipelineCompilationOptions::default(),
                }),
                primitive: PrimitiveState::default(),
                depth_stencil: None,
                multisample: MultisampleState::default(),
                multiview: None,
            });

        (pipeline, vertex_buffer, index_buffer)
    }
}
