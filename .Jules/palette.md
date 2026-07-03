## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2024-07-02 - Communicate Server-Side Form Validation Constraints to the User
**Learning:** Found that custom forms, like the "Custom Alias" input, were enforcing server-side constraints (length, specific characters) but failing to indicate these requirements to the user beforehand. This leads to frustrating "submit-fail-correct" cycles.
**Action:** Always provide inline helper text (`aria-describedby`) and apply client-side HTML5 validation attributes (`pattern`, `minlength`, `maxlength`) to inputs that have restrictive server-side validation rules.
