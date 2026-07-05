## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.
## 2024-07-02 - Form Helper text styling
**Learning:** When adding helper text to forms, DO NOT use inline styles, even if they are simple color and font size adjustments based on CSS variables. Always define a new CSS class or use an existing one in the `<style>` block of the HTML template.
**Action:** Always extract styling to CSS classes in `src/ui.rs` and avoid inline `style` attributes for maintainability and consistency.
