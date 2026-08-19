## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2026-07-07 - Prevent Loss of Form State with HTML5 Validation
**Learning:** Found that when the backend rejects form input (like invalid custom alias formats), users are redirected to a generic error page, losing their form state and context.
**Action:** Use HTML5 form validation (`pattern`, `minlength`, `maxlength`) matching backend constraints to provide immediate, inline feedback. This prevents unnecessary roundtrips and keeps the user in their current context without losing input data.
