# Mouse Handling and Window Resizing Cursor Fixes

## Issues Identified

### 1. Cursor Priority System Problems
**Problem**: Multiple windows requesting cursors simultaneously caused conflicts and inconsistent cursor display.

**Root Cause**: 
- The cursor priority system was resetting all cursor state each frame
- Equal priority cursors were not handled properly
- Debug output showed requests but final cursor wasn't applied correctly

**Fix Applied**:
- Modified `set_cursor_with_priority()` to prefer non-None cursors at equal priority
- Changed cursor reset logic to only reset low-priority cursors (< 5)
- Improved cursor state persistence across frames

### 2. Edge Resize Detection Issues
**Problem**: Edge detection was inconsistent and sometimes failed to trigger resize cursors.

**Root Cause**:
- Border size detection was too rigid
- No fallback for edge detection failures
- Detection area was too small for reliable interaction

**Fix Applied**:
- Expanded detection area beyond visible border (`detection_border = border_size.max(4.0)`)
- Added inner border calculation for more precise edge detection
- Improved corner vs edge prioritization

### 3. ImGui Integration Cursor Conflicts
**Problem**: ImGui manager was interfering with custom cursor management.

**Root Cause**:
- Manager was resetting cursor state each frame
- Custom cursor requests were overridden by ImGui's internal logic
- Cursor state wasn't properly preserved

**Fix Applied**:
- Removed automatic cursor reset in ImGui manager
- Let application manage cursor state through priority system
- Improved cursor mapping for additional cursor types

## Technical Changes Made

### `src/scene/basic_scene.rs`
- **Cursor Priority Logic**: Enhanced to handle equal priority cursors better
- **Cursor Reset**: Only reset low-priority cursors to maintain high-priority resize cursors
- **Cursor Mapping**: Added support for `ResizeAll` and `NotAllowed` cursors

### `src/ui/edge_resize.rs`
- **Detection Area**: Expanded beyond visible border for better interaction
- **Inner Border**: Added precise edge detection with smaller inner border
- **Cursor Consistency**: Always provide cursor feedback, even during resize operations
- **State Management**: Improved position and size tracking

### `src/ui/imgui_integration/manager.rs`
- **Cursor Management**: Removed automatic cursor reset to prevent conflicts
- **OS Integration**: Always let OS handle cursor drawing

### `src/main.rs`
- **Cursor Application**: Simplified cursor setting logic with proper fallback

## Expected Improvements

1. **Consistent Cursor Display**: Resize cursors should now appear reliably when hovering over window edges
2. **Better Edge Detection**: Larger detection areas make it easier to trigger resize operations
3. **No Cursor Conflicts**: Priority system prevents multiple windows from fighting over cursor control
4. **Smoother Interaction**: Cursor state persists properly during resize operations

## Testing Recommendations

1. **Edge Hover Test**: Move mouse along window edges to verify resize cursors appear
2. **Corner Detection**: Test corner resize cursors (diagonal arrows)
3. **Multi-Window Test**: Open multiple resizable windows and verify cursor behavior
4. **Resize Operation**: Start resize and verify cursor remains consistent during operation
5. **Window Lock Test**: Toggle window lock and verify cursor behavior changes appropriately

## Debugging Features

The fixes maintain existing debug output but reduce noise by only showing relevant cursor state changes. If issues persist, additional debugging can be enabled by uncommenting debug print statements in the cursor priority system.