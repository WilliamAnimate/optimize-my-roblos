const { invoke } = window.__TAURI__.tauri;

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
const alttweaksBtn = document.getElementById("btn-adv-alttweaks");
const vulkanVoxelBtn = document.getElementById("btn-adv-vulkanvoxel");
const mintweaks = document.getElementById("btn-adv-mintweaks");
const mintweaksNoVulkan = document.getElementById("btn-adv-mintweaks-novulkan");

async function tweak(element, funct) {
	putLoadingAnimationOnId(element);
	if (await invoke(funct) !== "we gud") {
		panic("Rust backend threw an error", basedOn);
	}
	showElementById(element);
}

optimizeBtn.addEventListener("click", function() {
	tweak(optimizeBtn, "optimize");
	showElementById(document.getElementById("done-txt"));
});

unoptimizeBtn.addEventListener("click", function() {
	tweak(unoptimizeBtn, "unoptimize");
});

// alt tweaks
alttweaksBtn.addEventListener("click", function() {
	tweak(alttweaksBtn, "optimize_alt_tweaks");
});
vulkanVoxelBtn.addEventListener("click", function() {
	tweak(vulkanVoxelBtn, "optimize_vulkanvoxel");
});
mintweaks.addEventListener("click", function() {
	tweak(mintweaks, "optimize_minimal")
});
mintweaksNoVulkan.addEventListener("click", function() {
	tweak(mintweaksNoVulkan, "optimize_minimal_novulkan");
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
