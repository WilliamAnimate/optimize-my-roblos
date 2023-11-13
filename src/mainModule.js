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

optimizeBtn.addEventListener("click", async function() {
	putLoadingAnimationOnId(optimizeBtn);
	await invoke("optimize");
	removeLoadingAnimationOnId(optimizeBtn);
	showElementById(document.getElementById("done-txt"));
});

unoptimizeBtn.addEventListener("click", async function() {
	putLoadingAnimationOnId(unoptimizeBtn);
	await invoke("unoptimize");
	removeLoadingAnimationOnId(unoptimizeBtn);
});

// alt tweaks
alttweaksBtn.addEventListener("click", async function() {
	putLoadingAnimationOnId(alttweaksBtn);
	await invoke("optimize_alt_tweaks");
	removeLoadingAnimationOnId(alttweaksBtn);
});
vulkanVoxelBtn.addEventListener("click", async function() {
	putLoadingAnimationOnId(vulkanVoxelBtn);
	await invoke("optimize_vulkanvoxel");
	removeLoadingAnimationOnId(vulkanVoxelBtn);
});
mintweaks.addEventListener("click", async function() {
	putLoadingAnimationOnId(mintweaks);
	await invoke("optimize_minimal");
	removeLoadingAnimationOnId(mintweaks);
});
mintweaksNoVulkan.addEventListener("click", async function() {
	putLoadingAnimationOnId(mintweaksNoVulkan);
	await invoke("optimize_minimal_novulkan");
	removeLoadingAnimationOnId(mintweaksNoVulkan);
});

invoke("get_version").then((result) => {
	document.getElementById("version").textContent = "v" + result;
}).catch((error) => {
	panic("[mainModule]: Failed to fetch version. This could indicate at a severe problem in the Rust backend.", error);
});

if (!pollDevelopmentMode()) {
	setTimeout(checkForNewVersion(), 1000);
} else {
	setTimeout(hideElementById(document.getElementById("cursor-wait-hbox")), 100);
}
