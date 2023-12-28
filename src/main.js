const invoke = window.__TAURI__.invoke
const btn_advanced = document.getElementById("btn-advanced");

/**development purposes only; so you don't ACTUALLY call the rust backend*/
// function invoke(a){console.log(`invoked function: ${a}`); return new Promise(resolve=>{resolve("0.69")})}

let develop = true;

window.addEventListener('contextmenu', e => {
	e.preventDefault();
});
window.addEventListener('keydown', e=>{if(!develop)e.preventDefault()});

/**
 * gets the last error.
 * @returns the raw promise from invoke() of the last error. you would want to retrive it with getLastError().then((e) => {doSomethingWith(e)})
 */
async function getLastError() {
	return await invoke("get_last_error");
}

function checkForNewVersion() {
	const xhr = new XMLHttpRequest();
	xhr.open("GET", "https://api.github.com/repos/williamanimate/optimize-my-roblos/releases/latest");
	xhr.setRequestHeader("User-Agent", "optimize_my_roblos"); // FIXME: lmaoooo don't do this

	xhr.onload = async function() {
		if (xhr.status !== 200) {
			hideElementById(document.getElementById("cursor-wait-hbox"));
			return;
		}

		const latest = JSON.parse(xhr.responseText).tag_name.slice(1);
		const result = await invoke("get_version");

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
	element.disabled = true;
	element.innerHTML = '<p>Please wait</p>';
	element.classList.add("cursor-wait");

	try {
		var result = await invoke(funct); // use var to keep it declared for the rest of the function
	} catch (e) {
		panic("Failed to call the optimize function, does it exist?", `${e}. Result info: ${result}`);
	}
	if (result !== true) {
		getLastError().then((e) => {
			panic("Backend threw an error", e);
		});
	}
	element.classList.remove("cursor-wait");
	element.innerHTML = "<p>Done!</p>";
}

const optimizeBtn = document.getElementById("btn-optimize");
const studioBtn = document.getElementById("btn-studio");
const unoptimizeBtn = document.getElementById("btn-unoptimize");
const nineteenSeventyFiveTweaksBtn = document.getElementById("btn-adv-1975");
const officeTweaksBtn = document.getElementById("btn-adv-office");
const perfTweaksBtn = document.getElementById("btn-adv-perf");
const optimizeGpuBtn = document.getElementById("btn-adv-optimizegpu");
const optimizeVinegarBtn = document.getElementById("btn-adv-unix-vinegar");

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
optimizeVinegarBtn.addEventListener("click", function() {
  tweak(optimizeVinegarBtn, "optimize_linux_vinegar");
})

function panic(title, message) {
	document.querySelectorAll('dialog').forEach(element => {
		element.close();
	});

	document.getElementById("errortitle").textContent = title;
	document.getElementById("errormessage").textContent = message;

	const panic = document.getElementById("panic");
	panic.classList.add("onTop");
	showElementById(panic);
	panic.style.opacity = 1;

	throw `error title: ${title}, message: ${message}`;
}

function unpanic() {
	console.log("recovering from panic now!");
	const panic = document.getElementById("panic");
	hideElementById(panic);
	panic.style.opacity = 0;
	panic.classList.remove("onTop");
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
	if (element == null) {
		panic("error in frontend", "element is null.");
	}

	element.style.opacity = '0';
	element.style.transform = 'translateY(20px)';

	setTimeout(function() {
		element.close();

		// reset
		element.style.opacity = '1';
		element.style.transform = 'translateY(0px)';
	}, 295);
}

function initSetTheme() {
	if (!develop || window.matchMedia('prefers-reduced-motion: reduce')) {
		setTimeout(function() {
			// microsoft webview sucks; it displays white for like 3 seconds before actually doing something
			// so we just ease it in
			document.body.style.backgroundColor = getComputedStyle(document.body).getPropertyValue('--color-bg');

			// adding fadein *might* cause a problem when changing the background (this function gets called again when prefers-color-scheme changes)
			document.body.classList.add("fadein");
			document.body.style.opacity = 1;
		}, 40);
	} else {
		document.body.style.backgroundColor = getComputedStyle(document.body).getPropertyValue('--color-bg');
		document.body.style.opacity = 1;
	}
}

initSetTheme();

// background is handled by js; we need this.
// attribution: https://stackoverflow.com/questions/59621784/how-to-detect-prefers-color-scheme-change-in-javascript
// TODO: one liner this someday
window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", initSetTheme());
window.matchMedia("(prefers-color-scheme: light)").addEventListener("change", initSetTheme());

btn_advanced.addEventListener("click", function() {
	openDialogById(document.getElementById("_d_advanced"));
});

invoke("get_version").then((ver) => {
	document.getElementById("version").textContent = "v" + ver;
});

if (develop)
	setTimeout(hideElementById(document.getElementById("cursor-wait-hbox")), 100);
else
	setTimeout(checkForNewVersion(), 1000);
