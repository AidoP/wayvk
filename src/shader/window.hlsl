struct vertex_in {
    float2 position: POSITION;
};

struct vertex_out {
    float4 position: SV_POSITION;
};

vertex_out vert(vertex_in input) {
    vertex_out output;
    output.position = float4(input.position, 0.0, 1.0);
    return output;
}

float4 frag(vertex_out output): SV_TARGET {
    return float4(1.0, 0.0, 0.0, 1.0);
}