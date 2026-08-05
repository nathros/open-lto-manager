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
function downloadFile(url, method, contentType, ext, body) {
	fetch(url, {
		method: method,
		headers: {
			"Content-Type": contentType,
		},
		body: body,
	})
		.then((response) => {
			if (!response.ok) {
				throw new Error(`Response status: ${response.status}`);
			}
			response.blob().then((blob) => {
				const url = window.URL.createObjectURL(blob);
				const link = document.createElement("a");
				link.href = url;
				let cd = response.headers.get("Content-Disposition");
				if (cd) {
					link.setAttribute("download", cd.split('"')[1]);
				} else {
					link.setAttribute("download", `file.${ext}`);
				}
				document.body.appendChild(link);
				link.click();
				document.body.removeChild(link);
			});
		})
		.catch((error) => {
			alert(`Download failed with error: ${error}`);
		});
}
