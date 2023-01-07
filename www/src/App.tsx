import reactLogo from "./assets/react.svg";
import "./App.css";
import { GameCanvas } from "./components/GameCanvas";
import { DirectCanvas } from "./components/DirectCanvas";

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
			<main>
				<DirectCanvas />
				{/* <GameCanvas /> */}
			</main>

		</div>
	);
}

export default App;
