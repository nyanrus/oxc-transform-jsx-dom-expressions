// JSX with dynamic content
function App() {
    const [name, setName] = createSignal("World");
    return <div>Hello {name()}!</div>;
}
