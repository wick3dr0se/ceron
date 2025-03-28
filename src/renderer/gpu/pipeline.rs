use wgpu::*;

use crate::vertex::Vertex;

use super::{context::RenderContext, shader::Shader};

pub(crate) struct Pipeline;

impl Pipeline {
    pub(crate) fn new(ctx: &RenderContext) -> RenderPipeline {
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
                    module: &Shader::load(&ctx, "position_color.wgsl"),
                    entry_point: Some("vertex_main"),
                    buffers: &[VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as BufferAddress,
                        step_mode: VertexStepMode::Vertex,
                        attributes: &Vertex::ATTRIBUTES,
                    }],
                    compilation_options: PipelineCompilationOptions::default(),
                },
                cache: None,
                fragment: Some(FragmentState {
                    module: &Shader::load(&ctx, "position_color.wgsl"),
                    entry_point: Some("fragment_main"),
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

        pipeline
    }
}
