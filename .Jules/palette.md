## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.

## 2026-07-17 - Align Frontend Validation with Backend Rules
**Learning:** In server-rendered applications, generic error pages can cause users to lose their form input state. Adding HTML5 validation attributes (, , ) that strictly match the backend logic ensures users receive immediate, inline feedback, preventing frustration and state loss.
**Action:** Always verify that frontend form inputs include corresponding HTML5 validation for all backend constraints.

## 2026-07-20 - Ensure A11y Visibility for Complex Validations
**Learning:** Adding strict regex or pattern validations (like `pattern="[a-zA-Z0-9\-_]+"`) to inputs without visible explanation frustrates users who encounter generic "Please match the requested format" errors. Screen reader users face even more difficulty if the constraints aren't read upfront.
**Action:** When adding strict validation rules to inputs, always provide a visible helper text detailing the constraints (e.g. allowed characters, length) and link it to the input using `aria-describedby` so the rules are announced when the input receives focus.

## $(date +%Y-%m-%d) - Align Frontend Validation with Backend Rules
**Learning:** In server-rendered applications, generic error pages can cause users to lose their form input state. Adding HTML5 validation attributes (`pattern`, `minlength`, `maxlength`) that strictly match the backend logic ensures users receive immediate, inline feedback, preventing frustration and state loss.
**Action:** Always verify that frontend form inputs include corresponding HTML5 validation for all backend constraints.
