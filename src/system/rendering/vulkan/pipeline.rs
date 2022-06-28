use std::ffi::CStr;
use std::io::Cursor;
use std::mem;
use std::default::Default;
use ash::{Device};
use ash::util::read_spv;
use ash::vk::{BlendFactor, BlendOp, ColorComponentFlags, CompareOp, DescriptorSetLayout, DynamicState, Extent2D, Format, FrontFace, GraphicsPipelineCreateInfo, LogicOp, Pipeline, PipelineCache, PipelineColorBlendAttachmentState, PipelineColorBlendStateCreateInfo, PipelineDepthStencilStateCreateInfo, PipelineDynamicStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PipelineLayout, PipelineLayoutCreateInfo, PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo, PipelineShaderStageCreateInfo, PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, RenderPass, SampleCountFlags, ShaderModule, ShaderModuleCreateInfo, ShaderStageFlags, StencilOp, StencilOpState, VertexInputAttributeDescription, VertexInputBindingDescription, VertexInputRate, Viewport};
use bytemuck::offset_of;
use crate::system::rendering::mesh::vertex::Vertex;

struct VkGraphicsPipeline {
    pipelines: Vec<Pipeline>,
    pipeline_layout: PipelineLayout,
    vertex_shader_mod: ShaderModule,
    fragment_shader_mod: ShaderModule,

}

impl VkGraphicsPipeline {
    pub fn new(device: &Device, render_pass: &RenderPass, desc_set_layout: &[DescriptorSetLayout], surface_size: Extent2D) -> Self {
        let mut vert_shader_cursor = Cursor::new(&include_bytes!("./shader/vert.spv")[..]);
        let mut frag_shader_cursor = Cursor::new(&include_bytes!("./shader/frag.spv")[..]);
        let vertex_shader_mod = VkGraphicsPipeline::create_shader_module(device, &mut vert_shader_cursor);
        let fragment_shader_mod = VkGraphicsPipeline::create_shader_module(device, &mut frag_shader_cursor);

        let layout_create_info = PipelineLayoutCreateInfo::builder().set_layouts(desc_set_layout);
        let pipeline_layout = unsafe { device.create_pipeline_layout(&layout_create_info, None).expect("failed to create pipeline layout") };

        let shader_entry_name = unsafe { CStr::from_bytes_with_nul_unchecked(b"main\0") };
        let shader_stage_create_infos = [
            PipelineShaderStageCreateInfo {
                module: vertex_shader_mod,
                p_name: shader_entry_name.as_ptr(),
                stage: ShaderStageFlags::VERTEX,
                ..Default::default()
            },
            PipelineShaderStageCreateInfo {
                module: fragment_shader_mod,
                p_name: shader_entry_name.as_ptr(),
                stage: ShaderStageFlags::FRAGMENT,
                ..Default::default()
            }
        ];

        let vertex_input_binding = [VertexInputBindingDescription {
            binding: 0,
            stride: mem::size_of::<Vertex>() as u32,
            input_rate: VertexInputRate::VERTEX,
        }];

        let vertex_input_attribute_descriptions = [
            VertexInputAttributeDescription {
                location: 0,
                binding: 0,
                format: Format::R32G32B32_SFLOAT,
                offset: offset_of!(Vertex, position) as u32,
            },
            VertexInputAttributeDescription {
                location: 1,
                binding: 0,
                format: Format::R32G32_SFLOAT,
                offset: offset_of!(Vertex, uv) as u32,
            },
        ];

        let vertex_input_state_info = PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
            .vertex_binding_descriptions(&vertex_input_binding);

        let vertex_input_assembly_state_info = PipelineInputAssemblyStateCreateInfo {
            topology: PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };

        let viewports = [Viewport {
            x: 0.0,
            y: 0.0,
            width: surface_size.width as f32,
            height: surface_size.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissors = [surface_size.into()];
        let viewport_state_info = PipelineViewportStateCreateInfo::builder()
            .scissors(&scissors)
            .viewports(&viewports);

        let rasterization_info = PipelineRasterizationStateCreateInfo {
            front_face: FrontFace::COUNTER_CLOCKWISE,
            line_width: 1.0,
            polygon_mode: PolygonMode::FILL,
            ..Default::default()
        };

        let multisample_state_info = PipelineMultisampleStateCreateInfo::builder().rasterization_samples(SampleCountFlags::TYPE_1);

        let noop_stencil_state = StencilOpState {
            fail_op: StencilOp::KEEP,
            pass_op: StencilOp::KEEP,
            depth_fail_op: StencilOp::KEEP,
            compare_op: CompareOp::ALWAYS,
            ..Default::default()
        };

        let depth_state_info = PipelineDepthStencilStateCreateInfo {
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: CompareOp::LESS_OR_EQUAL,
            front: noop_stencil_state,
            back: noop_stencil_state,
            max_depth_bounds: 1.0,
            ..Default::default()
        };

        let color_blend_attachment_states = [PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: BlendFactor::SRC_COLOR,
            dst_color_blend_factor: BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: BlendOp::ADD,
            src_alpha_blend_factor: BlendFactor::ZERO,
            dst_alpha_blend_factor: BlendFactor::ZERO,
            alpha_blend_op: BlendOp::ADD,
            color_write_mask: ColorComponentFlags::RGBA,
        }];

        let color_blend_state = PipelineColorBlendStateCreateInfo::builder()
            .logic_op(LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

        let dynamic_state = [DynamicState::VIEWPORT, DynamicState::SCISSOR];
        let dynamic_state_info = PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_state);

        let graphics_pipeline_infos = GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stage_create_infos)
            .vertex_input_state(&vertex_input_state_info)
            .input_assembly_state(&vertex_input_assembly_state_info)
            .viewport_state(&viewport_state_info)
            .rasterization_state(&rasterization_info)
            .multisample_state(&multisample_state_info)
            .depth_stencil_state(&depth_state_info)
            .color_blend_state(&color_blend_state)
            .dynamic_state(&dynamic_state_info)
            .layout(pipeline_layout)
            .render_pass(*render_pass);

        let pipelines = unsafe {
            device.create_graphics_pipelines(PipelineCache::null(),
                                             &[graphics_pipeline_infos.build()],
                                             None)
                .expect("Failed to create pipelines")
        };

        VkGraphicsPipeline {
            vertex_shader_mod,
            fragment_shader_mod,
            pipeline_layout,
            pipelines,
        }
    }

    pub fn get_primary(&self) -> &Pipeline {
        &self.pipelines[0]
    }

    fn create_shader_module(device: &Device, shader_file: &mut Cursor<&[u8]>) -> ShaderModule {
        let shader_code = read_spv(shader_file).expect("Failed to read vertex shader");
        let shader_info = ShaderModuleCreateInfo::builder().code(&shader_code);
        let shader_module = unsafe { device.create_shader_module(&shader_info, None).expect("failed to create shader module") };
        shader_module
    }
}
