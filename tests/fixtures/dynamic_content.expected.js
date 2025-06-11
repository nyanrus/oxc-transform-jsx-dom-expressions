// Expected output for dynamic_content.jsx
const _tmpl$1 = /*#__PURE__*/template(`<div>Hello <!--#-->!</div>`);

function App() {
    const [name, setName] = createSignal("World");
    return (() => {
        const _el$ = _tmpl$1();
        insert(_el$, name, null);
        return _el$;
    })();
}
