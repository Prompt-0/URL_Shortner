## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2024-07-12 - Inline Validation Attributes
**Learning:** Found that custom form fields in this app lacked HTML5 validation attributes, forcing users to rely on backend validation errors that navigated them away from the form state.
**Action:** Always verify that form inputs mirror backend constraints using standard HTML attributes (`minlength`, `maxlength`, `pattern`) to provide immediate, accessible inline feedback before form submission.
