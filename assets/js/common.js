function showModal(id) {
	document.getElementById(id).showModal();
}
function hidePopover(id) {
	document.getElementById(id).hidePopover();
}
function copyToClipboard(str) {
	if (navigator.clipboard) {
		navigator.clipboard.writeText(str);
	} else {
		try {
			const wrapper = document.createElement("div");
			document.body.appendChild(wrapper);
			const textarea = document.createElement("textarea");
			textarea.textContent = str;
			wrapper.appendChild(textarea);
			const selection = document.getSelection();
			const range = document.createRange();
			range.selectNodeContents(textarea);
			selection.removeAllRanges();
			selection.addRange(range);
			document.execCommand("copy");
			selection.removeAllRanges();
			document.body.removeChild(wrapper);
		} catch (e) {
			alert(`Failed to copy ${e}`);
		}
	}
}
