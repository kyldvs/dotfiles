local wezterm = require 'wezterm'
local mux = wezterm.mux

wezterm.on("gui-startup", function(cmd)
  local active = wezterm.gui.screens().active
  local tab, pane, window = mux.spawn_window(cmd or {
    x = active.x,
    y = active.y,
    width = active.width,
    height = active.height,
  })
  -- I don't think this is needed, but also set the positions after spawn.
  window:gui_window():set_position(active.x, active.y)
  window:gui_window():set_inner_size(active.width, active.height)
end)

return {
  color_scheme = 'Material Darker (base16)',
	enable_tab_bar = false,
	font_size = 14.0,
	font = wezterm.font('JetBrainsMono Nerd Font'),
  window_background_opacity = 1.0,
	window_decorations = 'RESIZE',
  mouse_bindings = {
    -- Ctrl-click will open the link under the mouse cursor
    {
      event = { Up = { streak = 1, button = 'Left' } },
      mods = 'CTRL',
      action = wezterm.action.OpenLinkAtMouseCursor,
    },
  },
}
