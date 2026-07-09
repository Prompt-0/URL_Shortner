## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2026-07-09 - Client-side Form Validation for Server-Rendered Error Pages
**Learning:** In a server-rendered app with basic HTML forms, encountering a backend validation error often navigates the user to a generic error page, losing their form state and context. Inline feedback isn't inherently preserved.
**Action:** Ensure HTML forms strictly use HTML5 validation attributes (`pattern`, `minlength`, `maxlength`) that mirror backend constraints to provide immediate, inline feedback and prevent destructive error navigation.
