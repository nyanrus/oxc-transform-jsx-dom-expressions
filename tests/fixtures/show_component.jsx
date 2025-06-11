// Solid.js Show component
function App() {
    const [visible, setVisible] = createSignal(true);
    return (
        <Show when={visible()} fallback={<div>Hidden</div>}>
            <div>Visible content</div>
        </Show>
    );
}
