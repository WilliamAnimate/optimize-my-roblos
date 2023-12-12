const invoke = window.__TAURI__.invoke
const btn_advanced = document.getElementById("btn-advanced");

// function invoke(a){console.log(a)}

let lastError; // this variable exists because im too lazy to make a function return promises.

let develop = true;

window.addEventListener('contextmenu', e => {
	e.preventDefault();
});
window.addEventListener('keydown', e=>{if(!develop)e.preventDefault()});

/**
 * call this function with `await`, then get the value of lastError.
 * @returns does not. read the value of lastError instead.
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

async function tweak(element, funct) {
	putLoadingAnimationOnId(element);
	try {
		var result = await invoke(funct); // use var to keep it declared for the rest of the function
		// FINALLY a use case for var.
	} catch (e) {
		panic("Failed to call the optimize function, does it exist?", e + ". Result info: " + result);
	}
	if (result !== true) {
		await getLastError();
		panic("Backend threw an error", lastError);
	}
	removeLoadingAnimationOnId(element);
}

const optimizeBtn = document.getElementById("btn-optimize");
const studioBtn = document.getElementById("btn-studio");
const unoptimizeBtn = document.getElementById("btn-unoptimize");
const nineteenSeventyFiveTweaksBtn = document.getElementById("btn-adv-1975");
const officeTweaksBtn = document.getElementById("btn-adv-office");
const perfTweaksBtn = document.getElementById("btn-adv-perf");
const optimizeGpuBtn = document.getElementById("btn-adv-optimizegpu");

optimizeBtn.addEventListener("click", async function() {
	await tweak(optimizeBtn, "optimize_lowspec"); // assume the user is on a slow machine, why else would they be here?
	showElementById(document.getElementById("done-txt"));
});

studioBtn.addEventListener("click", function() {
	tweak(studioBtn, "apply_studio_config_json");
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

function panic(title, message) {
	document.querySelectorAll('dialog').forEach(element => {
		element.remove();
	});
	document.getElementById('main-container').remove();

	document.getElementById("errortitle").textContent = title;
	document.getElementById("errormessage").textContent = message;

	const panic = document.getElementById("panic");
	panic.classList.add("onTop");
	showElementById(panic);
	panic.style.opacity = 1;
}

function hideElementById(element) {
	element.classList.add("fadeout");
	setTimeout(function() {
		element.classList.add("hidden");
		element.classList.remove("fadeout");
	}, 300);
}
function showElementById(element) {
	element.classList.remove("hidden");
	element.classList.add("fadein");
	setTimeout(function() {
		element.classList.remove("fadein");
	}, 300);
}

function openDialogById(element) {
	element.style.opacity = '0';
	element.style.transform = 'translateY(-20px)';

	element.showModal();
	element.style.opacity = '1';
	element.style.transform = 'translateY(0px)';

	// prevent the dialog from being at the bottom
	element.scrollTo(0, 0);
}

function closeDialogById(element) {
	element.style.opacity = '0';
	element.style.transform = 'translateY(20px)';

	setTimeout(function() {
		element.close();

		// reset
		element.style.opacity = '1';
		element.style.transform = 'translateY(0px)';
	}, 295);
}

function putLoadingAnimationOnId(element) {
	element.disabled = true;
	element.innerHTML = '<svg style="width:40px;height:40px"viewBox="0 0 16 16"><circle style="stroke:#63ADE5;fill:none;stroke-width:2px;stroke-linecap:round;transform-origin:50% 50%;animation:spin-infinite 2s linear infinite"cx="8px"cy="8px"r="7px"></circle></svg>';

	element.classList.add("cursor-wait");
}

function removeLoadingAnimationOnId(element, newText = "<p>Done!</p>") {
	element.classList.remove("cursor-wait");
	element.innerHTML = newText;
}

if (!develop || window.matchMedia('prefers-reduced-motion: reduce')) {
	setTimeout(function() {
		// microsoft webview sucks; it displays white for like 3 seconds before actually doing something
		// so we just ease it in
		document.body.style.backgroundColor = "#000";

		document.body.classList.add("fadein");
		document.body.style.opacity = 1;
	}, 40);
} else {
	document.body.style.backgroundColor = "#1b1b1b";
	document.body.style.opacity = 1;
}

btn_advanced.addEventListener("click", function() {
	openDialogById(document.getElementById("_d_advanced"));
});

invoke("get_version").then((ver) => {
	document.getElementById("version").textContent = "v" + ver;
}).catch((err) => {
	panic("[mainModule]: Failed to fetch version. This could indicate at a severe problem in the Rust backend.", err);
});

if (!develop) {
	setTimeout(checkForNewVersion(), 1000);
} else {
	setTimeout(hideElementById(document.getElementById("cursor-wait-hbox")), 100);
}
