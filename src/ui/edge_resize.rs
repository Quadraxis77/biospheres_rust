/// Custom edge resizing implementation for ImGui windows
use imgui::{Ui, WindowFlags, Condition};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ResizeEdge {
    None,
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct EdgeResizeState {
    pub is_resizing: bool,
    pub resize_edge: ResizeEdge,
    start_mouse_pos: [f32; 2],
    start_window_pos: [f32; 2],
    start_window_size: [f32; 2],
}

impl Default for EdgeResizeState {
    fn default() -> Self {
        Self {
            is_resizing: false,
            resize_edge: ResizeEdge::None,
            start_mouse_pos: [0.0, 0.0],
            start_window_pos: [0.0, 0.0],
            start_window_size: [0.0, 0.0],
        }
    }
}

impl EdgeResizeState {
    /// Detect which edge the mouse is hovering over
    pub fn detect_resize_edge(
        mouse_pos: [f32; 2],
        window_pos: [f32; 2],
        window_size: [f32; 2],
        border_size: f32,
    ) -> ResizeEdge {
        let [mx, my] = mouse_pos;
        let [wx, wy] = window_pos;
        let [ww, wh] = window_size;
        
        // Expand the detection area slightly beyond the visible border
        let detection_border = border_size.max(4.0);
        
        // Check if mouse is within the extended detection area
        if mx < wx - detection_border || mx > wx + ww + detection_border ||
           my < wy - detection_border || my > wy + wh + detection_border {
            return ResizeEdge::None;
        }
        
        // Use a smaller inner border for more precise edge detection
        let inner_border = border_size * 0.75;
        
        let near_left = mx <= wx + inner_border;
        let near_right = mx >= wx + ww - inner_border;
        let near_top = my <= wy + inner_border;
        let near_bottom = my >= wy + wh - inner_border;
        
        // Prioritize corner detection over edge detection
        let edge = match (near_left, near_right, near_top, near_bottom) {
            (true, false, true, false) => ResizeEdge::TopLeft,
            (false, true, true, false) => ResizeEdge::TopRight,
            (true, false, false, true) => ResizeEdge::BottomLeft,
            (false, true, false, true) => ResizeEdge::BottomRight,
            (true, false, false, false) => ResizeEdge::Left,
            (false, true, false, false) => ResizeEdge::Right,
            (false, false, true, false) => ResizeEdge::Top,
            (false, false, false, true) => ResizeEdge::Bottom,
            _ => ResizeEdge::None,
        };
        
        edge
    }
    
    /// Start a resize operation
    pub fn start_resize(
        &mut self,
        edge: ResizeEdge,
        mouse_pos: [f32; 2],
        window_pos: [f32; 2],
        window_size: [f32; 2],
    ) {
        if edge != ResizeEdge::None {
            self.is_resizing = true;
            self.resize_edge = edge;
            self.start_mouse_pos = mouse_pos;
            self.start_window_pos = window_pos;
            self.start_window_size = window_size;
        }
    }
    
    /// Update window size during resize
    pub fn update_resize(
        &self,
        current_mouse_pos: [f32; 2],
        min_size: [f32; 2],
    ) -> Option<([f32; 2], [f32; 2])> {
        if !self.is_resizing {
            return None;
        }
        
        let [mx, my] = current_mouse_pos;
        let [start_mx, start_my] = self.start_mouse_pos;
        let [start_wx, start_wy] = self.start_window_pos;
        let [start_ww, start_wh] = self.start_window_size;
        let [min_w, min_h] = min_size;
        
        let dx = mx - start_mx;
        let dy = my - start_my;
        
        let mut new_pos = self.start_window_pos;
        let mut new_size = self.start_window_size;
        
        match self.resize_edge {
            ResizeEdge::Left => {
                new_pos[0] = start_wx + dx;
                new_size[0] = (start_ww - dx).max(min_w);
                // Adjust position if we hit minimum width
                if new_size[0] == min_w {
                    new_pos[0] = start_wx + start_ww - min_w;
                }
            }
            ResizeEdge::Right => {
                new_size[0] = (start_ww + dx).max(min_w);
            }
            ResizeEdge::Top => {
                new_pos[1] = start_wy + dy;
                new_size[1] = (start_wh - dy).max(min_h);
                // Adjust position if we hit minimum height
                if new_size[1] == min_h {
                    new_pos[1] = start_wy + start_wh - min_h;
                }
            }
            ResizeEdge::Bottom => {
                new_size[1] = (start_wh + dy).max(min_h);
            }
            ResizeEdge::TopLeft => {
                new_pos[0] = start_wx + dx;
                new_pos[1] = start_wy + dy;
                new_size[0] = (start_ww - dx).max(min_w);
                new_size[1] = (start_wh - dy).max(min_h);
                // Adjust positions if we hit minimum sizes
                if new_size[0] == min_w {
                    new_pos[0] = start_wx + start_ww - min_w;
                }
                if new_size[1] == min_h {
                    new_pos[1] = start_wy + start_wh - min_h;
                }
            }
            ResizeEdge::TopRight => {
                new_pos[1] = start_wy + dy;
                new_size[0] = (start_ww + dx).max(min_w);
                new_size[1] = (start_wh - dy).max(min_h);
                // Adjust position if we hit minimum height
                if new_size[1] == min_h {
                    new_pos[1] = start_wy + start_wh - min_h;
                }
            }
            ResizeEdge::BottomLeft => {
                new_pos[0] = start_wx + dx;
                new_size[0] = (start_ww - dx).max(min_w);
                new_size[1] = (start_wh + dy).max(min_h);
                // Adjust position if we hit minimum width
                if new_size[0] == min_w {
                    new_pos[0] = start_wx + start_ww - min_w;
                }
            }
            ResizeEdge::BottomRight => {
                new_size[0] = (start_ww + dx).max(min_w);
                new_size[1] = (start_wh + dy).max(min_h);
            }
            ResizeEdge::None => return None,
        }
        
        Some((new_pos, new_size))
    }
    
    /// End resize operation
    pub fn end_resize(&mut self) {
        self.is_resizing = false;
        self.resize_edge = ResizeEdge::None;
    }
}

/// Custom window builder with edge resizing support
pub struct EdgeResizableWindow<'a> {
    title: &'a str,
    size: [f32; 2],
    position: [f32; 2],
    flags: WindowFlags,
    #[allow(dead_code)]
    resize_state: &'a mut EdgeResizeState,
    border_size: f32,
    min_size: [f32; 2],
}

impl<'a> EdgeResizableWindow<'a> {
    pub fn new(title: &'a str, resize_state: &'a mut EdgeResizeState) -> Self {
        Self {
            title,
            size: [300.0, 200.0],
            position: [100.0, 100.0],
            flags: WindowFlags::empty(),
            resize_state,
            border_size: 12.0,
            min_size: [100.0, 50.0],
        }
    }
    
    pub fn size(mut self, size: [f32; 2], _condition: Condition) -> Self {
        self.size = size;
        self
    }
    
    pub fn position(mut self, position: [f32; 2], _condition: Condition) -> Self {
        self.position = position;
        self
    }
    
    pub fn flags(mut self, flags: WindowFlags) -> Self {
        self.flags = flags;
        self
    }
    
    pub fn border_size(mut self, size: f32) -> Self {
        self.border_size = size;
        self
    }
    
    pub fn min_size(mut self, size: [f32; 2]) -> Self {
        self.min_size = size;
        self
    }
    
    pub fn build<F, C>(mut self, ui: &Ui, cursor_callback: C, f: F) -> bool
    where
        F: FnOnce(),
        C: FnOnce(Option<imgui::MouseCursor>),
    {

        let mouse_pos = ui.io().mouse_pos;
        
        // Simplified: Let ImGui handle all resizing, we just provide cursor feedback
        
        // Create the window with smart positioning
        let mut window_builder = ui.window(self.title);
        
        // Set initial size and position, let ImGui manage everything else
        window_builder = window_builder
            .size(self.size, Condition::FirstUseEver)
            .position(self.position, Condition::FirstUseEver);
        
        let window_token = window_builder
            .flags(self.flags) // Don't disable built-in resize - let ImGui handle it
            .begin();
            
        if let Some(_token) = window_token {
            // Get the actual window position and size from ImGui
            let actual_pos = ui.window_pos();
            let actual_size = ui.window_size();
            
            // More conservative approach to edge resizing
            // Disable edge resizing in these cases:
            let is_window_collapsed = ui.is_window_collapsed();
            let any_item_active = ui.is_any_item_active();
            let _any_item_hovered = ui.is_any_item_hovered();
            
            // Only disable cursor feedback for active content interaction
            // Don't disable for just hovering - that's too restrictive
            let disable_cursor_feedback = is_window_collapsed || 
                                         any_item_active;
            
            // Update position and size tracking
            self.position = actual_pos;
            self.size = actual_size;
            
            // Provide cursor feedback unless disabled
            let cursor = if disable_cursor_feedback {
                None
            } else {
                // Detect edges and provide appropriate cursor
                let edge = EdgeResizeState::detect_resize_edge(
                    mouse_pos,
                    self.position,
                    self.size,
                    self.border_size,
                );
                
                let cursor = Self::get_cursor_for_edge(edge);
                

                
                cursor
            };
            
            // Always call cursor callback with current cursor
            cursor_callback(cursor);
            
            // Debug: Simple test to see if we're getting any cursor feedback
            if cursor.is_some() {
                println!("Cursor feedback: {:?} for window '{}'", cursor, self.title);
            }
            
            // Render window content
            f();
            true
        } else {
            // Window not visible, no cursor needed
            cursor_callback(None);
            false
        }
    }
    
    /// Get the appropriate cursor for the resize edge
    fn get_cursor_for_edge(edge: ResizeEdge) -> Option<imgui::MouseCursor> {
        match edge {
            ResizeEdge::Left | ResizeEdge::Right => Some(imgui::MouseCursor::ResizeEW),
            ResizeEdge::Top | ResizeEdge::Bottom => Some(imgui::MouseCursor::ResizeNS),
            ResizeEdge::TopLeft | ResizeEdge::BottomRight => Some(imgui::MouseCursor::ResizeNWSE),
            ResizeEdge::TopRight | ResizeEdge::BottomLeft => Some(imgui::MouseCursor::ResizeNESW),
            ResizeEdge::None => None,
        }
    }
}