local wezterm = require 'wezterm'
local mux = wezterm.mux

-- Kind of works to maximize the window, but it's slow on mac. Using the
-- rectangle window manager to do this manually for now. For more discussion
-- see: https://github.com/wez/wezterm/issues/3299
-- wezterm.on("gui-startup", function(cmd)
-- 	local tab, pane, window = mux.spawn_window(cmd or {})
-- 	window:gui_window():maximize()
-- end)

return {
	color_scheme = 'Catppuccin Mocha',
	enable_tab_bar = false,
	font_size = 16.0,
	font = wezterm.font('JetBrains Mono'),

  -- macos_window_background_blur = 40,
	macos_window_background_blur = 30,

	-- window_background_image = '/Users/name/Downloads/wallpaper.jpg',
	-- window_background_image_hsb = {
	-- 	brightness = 0.01,
	-- 	hue = 1.0,
	-- 	saturation = 0.5,
	-- },

  window_background_opacity = 1.0,
	window_decorations = 'RESIZE',

  keys = {
		{
			key = 'f',
			mods = 'CTRL',
			action = wezterm.action.ToggleFullScreen,
		},
	},

  mouse_bindings = {
	  -- Ctrl-click will open the link under the mouse cursor
	  {
	    event = { Up = { streak = 1, button = 'Left' } },
	    mods = 'CTRL',
	    action = wezterm.action.OpenLinkAtMouseCursor,
	  },
	},
}
