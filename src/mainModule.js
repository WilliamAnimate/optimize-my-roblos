const { invoke } = window.__TAURI__.tauri;

let lastError; // this variable exists because im too lazy to make a function return promises.

/**
 * call this function with `await`, then get the value of lastError.
 * @returns does not.
 */
async function getLastError() {
	try {
		lastError = await invoke("get_last_error");
	} catch (e) {
		panic("i quit my job, the error handler can't even get an error", "WTF DID YOU DO??? " + e);
	}
}

function checkForNewVersion() {
	const xhr = new XMLHttpRequest();
	xhr.open("GET", "https://api.github.com/repos/williamanimate/optimize-my-roblos/releases");
	xhr.setRequestHeader("User-Agent", "optimize_my_roblos"); // FIXME: lmaoooo don't do this

	xhr.onload = async function() {
		if (xhr.status !== 200) {
			hideElementById(document.getElementById("cursor-wait-hbox"))
		}

		const releases = JSON.parse(xhr.responseText);
		const latest = releases[0].tag_name.slice(1);

		const result = await invoke("get_version");
		if (!/[^0-9]/.test(latest) || !/[^0-9]/.test(result)) {
			// assume that a version is like 1.0.0*-alpha* or something similar; don't try
			hideElementById(document.getElementById("cursor-wait-hbox"));
			return;
		}
		if (latest > result) {
			const update_text_container = document.getElementById("update_text_container");
			const update_bypass = document.getElementById("update_bypass");

			hideElementById(update_text_container);
			hideElementById(update_bypass);

			setTimeout(function() {
				update_text_container.textContent = `A new version is available: ${latest}`;

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

const optimizeBtn = document.getElementById("btn-optimize");
const unoptimizeBtn = document.getElementById("btn-unoptimize");
const nineteenSeventyFiveTweaksBtn = document.getElementById("btn-adv-1975");
const officeTweaksBtn = document.getElementById("btn-adv-office");
const perfTweaksBtn = document.getElementById("btn-adv-perf");
const optimizeGpuBtn = document.getElementById("btn-adv-optimizegpu");

async function tweak(element, funct) {
	putLoadingAnimationOnId(element);
	try {
		var result = await invoke(funct); // use var to keep it declared for the rest of the function
		// FINALLY a use case for var.
	} catch (e) {
		panic("Failed to call the optimize function, does it exist?", e + ". Result info: " + result);
		// print result here because the error of js is stored in result
		// ... i think.
	}
	if (result !== true) {
		await getLastError();
		panic("Backend threw an error", lastError);
	}
	removeLoadingAnimationOnId(element);
}

optimizeBtn.addEventListener("click", async function() {
	await tweak(optimizeBtn, "optimize_lowspec"); // assume the user is on a slow machine, why else would they be here?
	showElementById(document.getElementById("done-txt"));
});

unoptimizeBtn.addEventListener("click", function() {
	tweak(unoptimizeBtn, "unoptimize");
});

nineteenSeventyFiveTweaksBtn.addEventListener("click", function() {
	tweak(nineteenSeventyFiveTweaksBtn, "optimize_1975");
});
officeTweaksBtn.addEventListener("click", function() {
	tweak(officeTweaksBtn, "optimize_office");
});
perfTweaksBtn.addEventListener("click", function() {
	tweak(perfTweaksBtn, "optimize_perf");
});
optimizeGpuBtn.addEventListener("click", function() {
	tweak(optimizeGpuBtn, "optimize_gpu_settings");
});

invoke("get_version").then((ver) => {
	document.getElementById("version").textContent = "v" + ver;
}).catch((err) => {
	panic("[mainModule]: Failed to fetch version. This could indicate at a severe problem in the Rust backend.", err);
});

if (!pollDevelopmentMode()) {
	setTimeout(checkForNewVersion(), 1000);
} else {
	setTimeout(hideElementById(document.getElementById("cursor-wait-hbox")), 100);
}
