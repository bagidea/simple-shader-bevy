use bevy::{
    prelude::*,
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup,
            RenderPipelineDescriptor,
            ShaderRef, 
            SpecializedMeshPipelineError
        }
    },
    sprite::{
        Material2d,
        Material2dKey
    }
};

///////// GLSL //////////

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath, Component)]
pub struct GLSLMaterial {
    #[uniform(0)]
    pub i_time: f32,
    #[uniform(1)]
    pub i_resolution: Vec3
}

impl Material2d for GLSLMaterial {
    fn vertex_shader() -> ShaderRef {
       "shaders/simple_shader.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
       "shaders/simple_shader.frag".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: Material2dKey<Self>
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

///////// WGSL //////////

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WGSLMaterial {
    #[uniform(0)]
    pub i_time: f32,
    #[uniform(1)]
    pub i_resolution: Vec3
}

impl Material2d for WGSLMaterial {
    fn fragment_shader() -> ShaderRef {
       "shaders/simple_shader.wgsl".into()
    }
}