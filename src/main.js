let develop = true;

const btn_advanced = document.getElementById("btn-advanced");

function pollDevelopmentMode() {
	return develop;
}

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
		document.body.style.backgroundColor = "#1b1b1b";

		document.body.classList.add("fadein");
		document.body.style.opacity = 1;
	}, 40);
} else {
	document.body.style.backgroundColor = "#1b1b1b";
	document.body.style.opacity = 1;
}

window.addEventListener('contextmenu', e => {
	e.preventDefault();
});
window.addEventListener('keydown', e=>{if(!develop)e.preventDefault()});

btn_advanced.addEventListener("click", function() {
	openDialogById(document.getElementById("_d_advanced"));
});
