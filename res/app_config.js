{
    "window_config": {
        "xres": 1000,
        "yres": 1000,
		"resizable": false,
        "fullscreen": false,
		"title": "Snake"
    },
	"graphics": {
		"clear_color": {
			"r": 0.0,
			"g": 1.0,
			"b": 0.0
		},
		"vsync": true
	},
	"resources": {
		"textures": {
			"background_texture": "./textures/snake_bg.png"
		},
		"shaders": {
			"game_shader": {
				"vertex": "./shaders/GUIShader.vert",
				"fragment": "./shaders/GUIShader.frag"
			}
		}
	}
}