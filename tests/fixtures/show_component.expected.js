// Expected output for show_component.jsx
const _tmpl$1 = /*#__PURE__*/template(`<div>Hidden</div>`);
const _tmpl$2 = /*#__PURE__*/template(`<div>Visible content</div>`);

function App() {
    const [visible, setVisible] = createSignal(true);
    return createComponent(Show, {
        get when() {
            return visible();
        },
        fallback: _tmpl$1(),
        get children() {
            return _tmpl$2();
        }
    });
}
