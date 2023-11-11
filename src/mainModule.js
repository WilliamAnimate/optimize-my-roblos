const { invoke } = window.__TAURI__.tauri;

const optimizeBtn = document.getElementById("btn-optimize");

function checkForNewVersion() {
	const url = `https://api.github.com/repos/rust-lang/rust/releases/latest`;

	const xhr = new XMLHttpRequest(); // can js please cooperate and let me make this a one-liner
	xhr.open("GET", url);
	xhr.setRequestHeader("User-Agent", "optimize_my_roblos"); // FIXME: lmaoooo don't do this

	xhr.onload = function () {
		if (xhr.status === 200) {
			const latestVersion = JSON.parse(xhr.responseText).tag_name;
			let currentVersion;

			invoke("get_version").then((result) => {
				currentVersion = result;

				// due to async, all the update check code will be put here.

				if (latestVersion !== currentVersion) {
					const update_text_container = document.getElementById("update_text_container");
					const update_bypass = document.getElementById("update_bypass");

					hideElementById(update_text_container);
					hideElementById(update_bypass);

					setTimeout(function() {
						update_text_container.textContent = `A new version is available: ${latestVersion}`; // IMPORTANT: use textContent instead of innerHTML to prevent random <script> execution, IF github gets hacked

						update_bypass.innerHTML = "<p>ok</p>";

						showElementById(update_text_container);
						showElementById(update_bypass);
					}, 500);
				} else {
					hideElementById(document.getElementById("cursor-wait-hbox"));
				}
			})
			.catch((error) => {
				panic("[mainModule]: failed to check for updates", error);
			});
		} else {
			// github is down, ignore
		}
	};

	xhr.onerror = function() {
		// network error
		panic("[mainModule]: network error, unable to fetch updates", "title says it all");
	};

	xhr.send();
}

optimizeBtn.addEventListener("click", function() {
	optimizeBtn.disabled = true;

	optimizeBtn.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg"xmlns:xlink="http://www.w3.org/1999/xlink"style="width:40px;height:40px"viewBox="0 0 16 16"><circle style="stroke:#63ADE5;fill:none;stroke-width:2px;stroke-linecap:round;transform-origin:50% 50%;animation:spin-infinite 2s linear infinite"cx="8px"cy="8px"r="7px"></circle></svg>';

	optimizeBtn.classList.add("cursor-wait");

	invoke("optimize");

	let a = setInterval(function() {
		console.log("A");
		if (invoke("queue_status")) {
			clearInterval(a);

			console.log("done");
			optimizeBtn.classList.remove("cursor-wait");
			optimizeBtn.innerHTML = "<p>Done!</p>"
		}

	}, 1000);
});

invoke("get_version").then((result) => {
	document.getElementById("version").textContent = result;
}).catch((error) => {
	panic("[mainModule]: Failed to fetch version. This could indicate at a severe problem at the Rust backend.", error);
});

if (!pollDevelopmentMode()) {
	setTimeout(function() {
		checkForNewVersion();
	}, 1000);
} else {
	setTimeout(function() {
		hideElementById(document.getElementById("cursor-wait-hbox"))
	}, 100);
}
