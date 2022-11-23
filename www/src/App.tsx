import reactLogo from "./assets/react.svg";
import "./App.css";
import { GameCanvas } from "./components/GameCanvas";
import { Greeter } from "./components/Greeter";

function App() {
	return (
		<div className="App">
			<div>
				<a href="https://vitejs.dev" target="_blank" rel="noreferrer">
					<img src="/vite.svg" className="logo" alt="Vite logo" />
				</a>
				<a href="https://reactjs.org" target="_blank" rel="noreferrer">
					<img src={reactLogo} className="logo react" alt="React logo" />
				</a>
			</div>
			<h1>Vite + React</h1>
			<div style={{display: "flex", flexDirection: "column", gap: "2rem"}}>
				<GameCanvas />
			</div>

		</div>
	);
}

export default App;
