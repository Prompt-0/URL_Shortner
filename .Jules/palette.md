## 2024-07-01 - Enhance Form Indicators and Keyboard Navigation
**Learning:** Found that custom UI components (like glassmorphism cards and stylized links/buttons) in this URL shortener lacked `:focus-visible` states, which makes keyboard navigation difficult. Also, required form fields were missing visual indicators for users, making form validation less apparent.
**Action:** Always verify that custom stylized buttons and links include `:focus-visible` styles that contrast well with their backgrounds. Ensure required form inputs explicitly mark their necessity visually, like with an asterisk.
## 2024-05-18 - Inline Form Validation
**Learning:** Adding HTML5 validation attributes (`pattern`, `minlength`, `maxlength`, `title`) to inputs that strictly match backend constraints prevents unnecessary round-trips to the server and loss of form state on generic error pages.
**Action:** Always ensure client-side HTML5 validation mirrors backend constraints for a smoother user experience, particularly in server-rendered applications where failed submissions might lead to full page reloads or generic error screens.
