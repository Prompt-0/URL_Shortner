## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2024-07-08 - Use HTML5 Form Validation in Server-Rendered Apps
**Learning:** In server-rendered apps (like axum with HTML string responses), purely backend form validation causes users to lose their form state when a generic error page is returned. Relying only on backend errors leads to a frustrating UX where users must retype everything if they make a small mistake.
**Action:** Always map backend validation constraints (length, patterns, required fields) to HTML5 frontend validation attributes (`minlength`, `maxlength`, `pattern`, `required`, `title`) to provide immediate, inline feedback and prevent form state loss.
