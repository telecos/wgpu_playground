# Accessibility Features

This document outlines the accessibility features implemented in wgpu_playground to ensure the application is usable by everyone, including users with disabilities.

## Overview

wgpu_playground has been designed with accessibility in mind, following WCAG 2.1 AA guidelines where applicable. The application includes features for keyboard navigation, screen reader support, and sufficient color contrast.

## HTML/Web Interface Accessibility

### Semantic HTML
- Proper HTML5 semantic elements (`<header>`, `<main>`, `<footer>`, `<nav>`)
- Appropriate heading hierarchy (h1 for page title, h2 for section headings)
- Descriptive page title and meta description for SEO and screen readers
- Language attribute (`lang="en"`) on HTML element

### ARIA Labels and Roles
- **Skip Navigation Link**: Allows keyboard users to skip directly to main content
- **ARIA Labels**: All interactive elements have descriptive labels
  - Canvas: `role="img"` with descriptive label explaining its purpose
  - Loading status: `role="status"` with `aria-live="polite"`
  - Error messages: `role="alert"` with `aria-live="assertive"`
  - Navigation: `aria-label` on footer navigation

### Keyboard Navigation
The web interface supports full keyboard navigation:

- **Tab/Shift+Tab**: Navigate between interactive elements
- **Enter/Space**: Activate canvas interaction mode
- **Escape**: Exit canvas interaction mode or close help modal
- **H**: Show/hide keyboard shortcuts help modal

When the canvas receives focus, screen readers announce available keyboard shortcuts. The help modal provides a comprehensive list of available keyboard shortcuts and can be accessed at any time.

### Focus Indicators
- High-contrast focus indicators (3px solid yellow outline with 2px offset)
- Clear visual feedback when elements receive keyboard focus
- Consistent focus styling across all interactive elements

### Color Contrast
All text and interactive elements meet WCAG AA standards (4.5:1 contrast ratio):
- Error messages: Dark red (#b30000) on light pink background
- Info sections: Dark text (#1a1a1a) on light blue background
- Links: White text with yellow focus state for high contrast
- Border colors: Higher contrast variants for better visibility

### Screen Reader Support
- `lang="en"` attribute on HTML element for proper language identification
- Descriptive meta tags for page context
- ARIA live regions for dynamic content updates
- Hidden decorative elements (`aria-hidden="true"` on separators)
- Screen reader only content with `.sr-only` class
- Modal dialogs with proper ARIA attributes (`role="dialog"`, `aria-modal="true"`, `aria-labelledby`)

## Desktop Application (egui) Accessibility

### Keyboard Shortcuts

#### File Operations
- **Ctrl+S** (Cmd+S on Mac): Save playground state
- **Ctrl+O** (Cmd+O on Mac): Load playground state

#### Quick Navigation
- **Ctrl+1** (Cmd+1 on Mac): Jump to Rendering panel
- **Ctrl+2** (Cmd+2 on Mac): Jump to Compute panel
- **Ctrl+3** (Cmd+3 on Mac): Jump to Buffers panel
- **Ctrl+4** (Cmd+4 on Mac): Jump to Textures panel
- **Ctrl+5** (Cmd+5 on Mac): Jump to Console panel
- **Ctrl+6** (Cmd+6 on Mac): Jump to Settings panel

### Tooltips and Descriptions
All UI elements include hover tooltips that provide:
- Descriptions of what the element does
- Keyboard shortcuts (where applicable)
- Additional context for complex features

Examples:
- "Save current playground state to a file (Ctrl+S)"
- "Configure GPU adapter and device settings"
- "View rendering examples and live preview"

### Navigation Structure
The application uses a clear, hierarchical navigation structure:
- Collapsible sections with clear labels
- Visual indicators for section state (open/closed)
- Consistent indentation for sub-items
- Icon prefixes for quick visual identification

### Default Accessibility in egui
The egui framework provides built-in accessibility features:
- Full keyboard navigation using Tab, Arrow keys, Enter, and Escape
- Default themes meet WCAG AA contrast requirements
- Proper focus management and visual feedback
- Logical tab order through UI elements

## Additional Accessibility Considerations

### Error Handling
- Clear, descriptive error messages
- Visual and programmatic distinction between errors and other messages
- Errors use ARIA live regions for immediate screen reader announcement

### Progressive Disclosure
- Collapsible sections reduce cognitive load
- Users can expand only the sections they need
- State is preserved across sessions

### Cross-Platform Support
- Keyboard shortcuts use platform-appropriate modifiers (Ctrl on Windows/Linux, Cmd on Mac)
- UI scales appropriately on different screen sizes
- Supports high DPI displays

## Testing Recommendations

To ensure continued accessibility:

1. **Keyboard Navigation Testing**
   - Navigate through all UI elements using only the keyboard
   - Verify all interactive elements are reachable
   - Ensure visual focus indicator is always visible

2. **Screen Reader Testing**
   - Test with NVDA (Windows), JAWS (Windows), or VoiceOver (Mac)
   - Verify all content is announced correctly
   - Check ARIA labels are meaningful

3. **Color Contrast Testing**
   - Use tools like WebAIM's Contrast Checker
   - Verify all text meets WCAG AA standards (4.5:1)
   - Test with color blindness simulators

4. **Zoom Testing**
   - Test at 200% zoom level
   - Verify no content is cut off or overlapping
   - Ensure functionality remains intact

## Future Improvements

Potential areas for further accessibility enhancements:
- High contrast theme option
- Customizable keyboard shortcuts
- Font size adjustment controls
- Reduced motion mode for animations
- More comprehensive screen reader announcements for WebGPU operations

## Compliance

This application strives to meet:
- **WCAG 2.1 Level AA** for web content
- **Section 508** standards where applicable
- Best practices for desktop application accessibility

## Resources

- [WCAG 2.1 Guidelines](https://www.w3.org/WAI/WCAG21/quickref/)
- [WebAIM Accessibility Resources](https://webaim.org/)
- [egui Accessibility Documentation](https://docs.rs/egui/latest/egui/)

## Reporting Issues

If you encounter any accessibility issues, please report them on our [GitHub Issues page](https://github.com/telecos/wgpu_playground/issues) with the label "accessibility".
