## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2024-07-10 - Add HTML5 Validation for Better Server-Rendered App UX
**Learning:** In server-rendered applications, generic error pages for invalid inputs lose user's form state. Utilizing built-in HTML5 validation (like `pattern`, `minlength`, `maxlength`) matched to backend constraints provides instant, inline feedback before submission.
**Action:** Always replicate critical backend validation rules on HTML forms using HTML5 attributes for an improved, frictionless UX that preserves form state on failure.
