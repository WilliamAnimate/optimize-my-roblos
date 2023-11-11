const { invoke } = window.__TAURI__.tauri;

const optimizeBtn = document.getElementById("btn-optimize");

function checkForNewVersion() {
	const xhr = new XMLHttpRequest(); // can js please cooperate and let me make this a one-liner
	xhr.open("GET", "https://api.github.com/repos/williamanimate/optimize-my-roblos/releases/latest");
	xhr.setRequestHeader("User-Agent", "optimize_my_roblos"); // FIXME: lmaoooo don't do this

	xhr.onload = async function() {
		if (xhr.status !== 200) {
			hideElementById(document.getElementById("cursor-wait-hbox"))
		}

		const latest = JSON.parse(xhr.responseText).tag_name;

		const result = await invoke("get_version");
		if (latest !== result) {
			const update_text_container = document.getElementById("update_text_container");
			const update_bypass = document.getElementById("update_bypass");

			hideElementById(update_text_container);
			hideElementById(update_bypass);

			setTimeout(function() {
				update_text_container.textContent = `A new version is available: ${latest}`; // IMPORTANT: use textContent instead of innerHTML to prevent random <script> execution, IF github gets hacked

				update_bypass.innerHTML = "<p>ok</p>";

				showElementById(update_text_container);
				showElementById(update_bypass);
			}, 500);
		} else {
			hideElementById(document.getElementById("cursor-wait-hbox"));
		}
	}

	xhr.send();
}

optimizeBtn.addEventListener("click", async function() {
	optimizeBtn.disabled = true;
	optimizeBtn.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg"xmlns:xlink="http://www.w3.org/1999/xlink"style="width:40px;height:40px"viewBox="0 0 16 16"><circle style="stroke:#63ADE5;fill:none;stroke-width:2px;stroke-linecap:round;transform-origin:50% 50%;animation:spin-infinite 2s linear infinite"cx="8px"cy="8px"r="7px"></circle></svg>';

	optimizeBtn.classList.add("cursor-wait");

	await invoke("optimize");
	optimizeBtn.classList.remove("cursor-wait");
	optimizeBtn.innerHTML = "<p>Done!</p>"
	showElementById(document.getElementById("done-txt"));
});

invoke("get_version").then((result) => {
	document.getElementById("version").textContent = "v" + result;
}).catch((error) => {
	panic("[mainModule]: Failed to fetch version. This could indicate at a severe problem at the Rust backend.", error);
});

if (!pollDevelopmentMode()) {
	setTimeout(checkForNewVersion(), 1000);
} else {
	setTimeout(hideElementById(document.getElementById("cursor-wait-hbox")), 100);
}
