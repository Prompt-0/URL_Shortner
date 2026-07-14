## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2024-07-28 - Prevent Form State Loss with HTML5 Validation
**Learning:** Found that when server-rendered apps only validate inputs on the backend, any validation failure (like an invalid custom alias format) causes the user to land on a generic error page and lose their entire form state.
**Action:** Always map backend validation constraints (like length limits or regex patterns) to strict HTML5 validation attributes (`minlength`, `maxlength`, `pattern`) on the frontend inputs. This ensures inline feedback and prevents accidental form state loss.
