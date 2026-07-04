## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2024-08-01 - Sync Frontend/Backend Validation & Helper Text Accessibility
**Learning:** Forms often perform validation only on the backend, leading to poor UX since users wait for a full server round-trip to learn of simple formatting errors. Additionally, validation constraints are often invisible until an error occurs.
**Action:** Always sync backend validation rules (like length and character constraints) to the frontend using HTML5 validation attributes (`pattern`, `minlength`, `maxlength`). Provide clear, visible helper text explaining these rules and associate the text with the input using `aria-describedby` to ensure screen reader users receive the same guidance.
