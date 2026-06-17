let systemCanvas = new (function () {
	// Isolates this code from other JavaScript

	let showGrid = false; // Debug, toggle with ` Tilde key.
	let frameTime;

	let speedIncrease = 2; // Frame time decrease amount when object gets bigger.
	let frameTimeStart = 100; // How long a frame lasts in milliseconds, adjust speed.
	let frameTimeMin = 20; // Minimum frame time.

	let direction = {
		LEFT: "LEFT",
		UP: "UP",
		RIGHT: "RIGHT",
		DOWN: "DOWN",
	};
	let sysDirection;
	let sys = [];
	let cellsXNum = 36;
	let cellsYNum = 24;
	let square;
	let sysActive = false;
	let sysEnd = false;
	let showSplash = true;
	let acceptInput = true;
	let score = 0;

	let canvas;
	let ctx;
	let drawInterval;
	let bodyOverflowOriginal;

	//createSystemCanvas();

	function createSystemCanvas() {
		// Create canvas, is exists then replace.
		canvas = document.getElementById("sys-canvas");
		if (canvas != null) {
			destroySys();
		}
		canvas = document.createElement("canvas");
		ctx = canvas.getContext("2d");
		canvas.id = "sys-canvas";
		canvas.width = window.innerWidth;
		canvas.height = window.innerHeight;
		canvas.style.top = "0px";
		canvas.style.left = "0px";
		canvas.style.width = "100%";
		canvas.style.height = "100%";
		canvas.style.position = "fixed";
		bodyOverflowOriginal = document.body.style.overflow;
		document.body.style.overflow = "hidden";
		canvas.style.zIndex = "1000";
		document.body.appendChild(canvas);
		window.addEventListener("keydown", sysKeyDown);
		window.addEventListener("resize", draw);
		canvas.addEventListener("click", click, false);
		canvas.addEventListener("mousemove", mouse, false);
		init();
		draw(false);
	}

	function init() {
		frameTime = frameTimeStart;
		sysEnd = false;
		acceptInput = true;
		score = 0;
		sysDirection = direction.RIGHT;
		sys = [];
		sys.push({
			x: Math.floor(cellsXNum / 4),
			y: Math.floor(cellsYNum / 4),
		});
		square = { x: 0, y: 0 };
	}

	function mouse(e) {
		// Change mouse on cross top right corner.
		let yPos = e.pageY - window.scrollY;
		let xPos = e.pageX - window.scrollX;
		let cellSize = getCellSize();
		cellSize = cellSize > 24 ? cellSize : 24;
		cellSize = cellSize * 1.5 * 1.25;
		if (
			xPos > window.innerWidth - cellSize &&
			yPos - cellSize / 4 < cellSize
		)
			canvas.style.cursor = "pointer";
		else canvas.style.cursor = "default";
	}

	function click(e) {
		// Click cross to close.
		let yPos = e.pageY - window.scrollY;
		let xPos = e.pageX - window.scrollX;
		let cellSize = getCellSize();
		cellSize = cellSize > 24 ? cellSize : 24;
		cellSize = cellSize * 1.5 * 1.25;
		if (
			xPos > window.innerWidth - cellSize &&
			yPos - cellSize / 4 < cellSize
		)
			destroySys();
	}

	function newSquare() {
		let xf = Math.floor(Math.random() * cellsXNum);
		let yf = Math.floor(Math.random() * cellsYNum);

		while (collision(xf, yf)) {
			xf++;
			if (xf > cellsXNum) {
				xf = 0;
				yf++;
			}
			if (yf > cellsYNum) {
				yf = 0;
			}
		}

		square = {
			x: xf,
			y: yf,
		};
		// Speed up every time sys gets bigger.
		frameTime -= speedIncrease;
		if (frameTime < frameTimeMin) frameTime = frameTimeMin;
	}

	function destroySys() {
		let c = document.getElementById("sys-canvas");
		if (c != null) c.parentElement.removeChild(c); // Better: c.remove(); But IE strikes again.
		window.removeEventListener("keydown", sysKeyDown);
		window.removeEventListener("resize", draw);
		window.removeEventListener("click", click);
		window.removeEventListener("mousemove", mouse);
		window.clearInterval(drawInterval);
		sysActive = false;
		sysEnd = false;
		showSplash = true;
		document.body.style.overflow = bodyOverflowOriginal;
	}

	function restartSys() {
		window.clearInterval(drawInterval);
		init();
		newSquare();
		drawInterval = window.setTimeout(draw, frameTime, true); // Start, draw calls update state.
	}

	function drawSplash() {
		ctx.save();
		ctx.shadowColor = "white";
		ctx.shadowBlur = 3;
		let cellSize = getCellSize();
		ctx.font = "bold " + cellSize + "px Lucida Sans Unicode";
		let yPos = Math.floor(window.innerHeight / 2);

		let t1 = atob("WW91IGhhdmUgc3R1bWJsZWQgdXBvbi4uLg==");
		let t1w = ctx.measureText(t1).width;
		let t2 = atob("Li4uYSBoaWRkZW4gZ2FtZSBvZiBzbmFrZQ==");
		let t2w = ctx.measureText(t2).width;

		ctx.fillText(
			t1,
			Math.floor((window.innerWidth - t1w) / 2),
			yPos - cellSize * 2
		);
		ctx.fillText(
			t1,
			Math.floor((window.innerWidth - t1w) / 2),
			yPos - cellSize * 2
		);
		ctx.fillText(
			t2,
			Math.floor((window.innerWidth - t2w) / 2),
			yPos - cellSize + 5
		);
		ctx.fillText(
			t2,
			Math.floor((window.innerWidth - t2w) / 2),
			yPos - cellSize + 5
		);

		ctx.font = cellSize + "px Lucida Sans Unicode";
		let t3 = atob("UHJlc3MgW1NwYWNlXSB0byBzdGFydA==");
		let t3w = ctx.measureText(t3).width;
		ctx.fillText(
			t3,
			Math.floor((window.innerWidth - t3w) / 2),
			yPos + cellSize * 2
		);
		ctx.fillText(
			t3,
			Math.floor((window.innerWidth - t3w) / 2),
			yPos + cellSize * 2
		);

		let t4 =
			atob("Q29udHJvbHM6IEFycm93IGtleXMg") +
			"[\u2190][\u2192][\u2191][\u2193]" +
			atob("IG9yIFtXXVtBXVtTXVtEXQ==");
		let t4w = ctx.measureText(t4).width;
		let y = Math.floor(
			(window.innerHeight - cellsYNum * cellSize) / 2 +
				cellSize * cellsYNum -
				cellSize / 2
		);
		ctx.fillText(t4, Math.floor((window.innerWidth - t4w) / 2), y);
		ctx.fillText(t4, Math.floor((window.innerWidth - t4w) / 2), y);
		ctx.restore();
	}

	function sysOver() {
		sysActive = false;
		sysEnd = true;
		window.clearInterval(drawInterval);
		ctx.save();
		ctx.shadowColor = "white";
		ctx.shadowBlur = 3;
		let cellSize = getCellSize();
		ctx.font = "bold " + cellSize + "px Lucida Sans Unicode";
		let yPos = Math.floor(window.innerHeight / 2);

		let t2 = atob("R0FNRSBPVkVS");
		let t2w = ctx.measureText(t2).width;
		ctx.fillText(
			t2,
			Math.floor((window.innerWidth - t2w) / 2),
			yPos - cellSize + 5
		);
		ctx.fillText(
			t2,
			Math.floor((window.innerWidth - t2w) / 2),
			yPos - cellSize + 5
		);
		ctx.beginPath();

		ctx.font = cellSize + "px Lucida Sans Unicode";
		let t3 = atob("W1NwYWNlXSByZXN0YXJ0LCBbRXNjXSBleGl0");
		let t3w = ctx.measureText(t3).width;
		ctx.fillText(
			t3,
			Math.floor((window.innerWidth - t3w) / 2),
			yPos + cellSize * 2
		);
		ctx.fillText(
			t3,
			Math.floor((window.innerWidth - t3w) / 2),
			yPos + cellSize * 2
		);

		let t4 = atob("U2NvcmU6IA==") + score;
		let t4w = ctx.measureText(t4).width;
		let y = Math.floor(
			(window.innerHeight - cellsYNum * cellSize) / 2 +
				cellSize * cellsYNum -
				cellSize / 2
		);
		ctx.fillText(t4, Math.floor((window.innerWidth - t4w) / 2), y);
		ctx.fillText(t4, Math.floor((window.innerWidth - t4w) / 2), y);
		ctx.restore();
	}

	function sysKeyDown(e) {
		switch (e.which) {
			case 65: // A.
			case 37: // left Arrow.
				if (acceptInput)
					if (sys.length < 2 || sysDirection != direction.RIGHT)
						sysDirection = direction.LEFT;
				acceptInput = false;
				break;

			case 68: // D.
			case 39: // Right Arrow.
				if (acceptInput)
					if (sys.length < 2 || sysDirection != direction.LEFT)
						sysDirection = direction.RIGHT;
				acceptInput = false;
				break;

			case 87: // W.
			case 38: // Up Arrow.
				if (acceptInput)
					if (sys.length < 2 || sysDirection != direction.DOWN)
						sysDirection = direction.UP;
				acceptInput = false;
				break;

			case 83: // S.
			case 40: // Down Arrow.
				if (acceptInput)
					if (sys.length < 2 || sysDirection != direction.UP)
						sysDirection = direction.DOWN;
				acceptInput = false;
				break;

			case 32: // Space.
				restartSys();
				sysActive = true;
				showSplash = false;
				break;

			case 27: // Escape.
				destroySys();
				break;

			case 223: // Tilde.
			case 192:
				showGrid = !showGrid;
				draw(false);
				break;
		}
	}

	function getCellSize() {
		let xCellSize = Math.floor(window.innerWidth / (cellsXNum + 2));
		let yCellSize = Math.floor(window.innerHeight / (cellsYNum + 2));
		return xCellSize <= yCellSize ? xCellSize : yCellSize;
	}

	function collision(x, y) {
		if (x < 0 || x > cellsXNum - 1 || y < 0 || y > cellsYNum - 1)
			return true; // Edge of area.
		let i;
		for (i = 0; i < sys.length - 1; i++) {
			// Sys with it self, not last segment as this will move.
			if (sys[i].x == x && sys[i].y == y) return true;
		}
		return false;
	}

	function updateSysState() {
		let head = {
			x: 0,
			y: 0,
		};
		head.x = sys[0].x;
		head.y = sys[0].y;

		switch (sysDirection) {
			case direction.LEFT:
				head.x--;
				break;

			case direction.RIGHT:
				head.x++;
				break;

			case direction.UP:
				head.y--;
				break;

			case direction.DOWN:
				head.y++;
				break;
		}

		if (collision(head.x, head.y)) return true;

		sys.splice(0, 0, head);
		if (head.x == square.x && head.y == square.y) {
			score += 10;
			sys = sys.slice(0, sys.length);
			if (cellsXNum * cellsYNum == sys.length) {
				sysOver();
			} else {
				newSquare();
			}
		} else {
			sys = sys.slice(0, sys.length - 1);
		}
		acceptInput = true;
	}

	function drawRoundRect(x, y, cellSize, r) {
		ctx.beginPath();
		ctx.moveTo(x + r, y);
		ctx.arcTo(x + cellSize, y, x + cellSize, y + cellSize, r);
		ctx.arcTo(x + cellSize, y + cellSize, x, y + cellSize, r);
		ctx.arcTo(x, y + cellSize, x, y, r);
		ctx.arcTo(x, y, x + cellSize, y, r);
		ctx.closePath();
		ctx.fill();
	}

	function draw(advanceSysState) {
		if (sysActive && advanceSysState) {
			if (updateSysState()) {
				sysOver();
				return;
			}
		}

		let ratio = window.devicePixelRatio;
		if (ratio != 1) {
			ratio = window.devicePixelRatio;
			canvas.width = Math.ceil(window.innerWidth * ratio);
			canvas.height = Math.ceil(window.innerHeight * ratio);
			canvas.getContext("2d").scale(ratio, ratio);
		} else {
			canvas.width = window.innerWidth;
			canvas.height = window.innerHeight;
		}

		let cellSize = getCellSize();

		let xStart = Math.floor((window.innerWidth - cellsXNum * cellSize) / 2);
		let yStart = Math.floor(
			(window.innerHeight - cellsYNum * cellSize) / 2
		);

		ctx.clearRect(0, 0, canvas.width, canvas.height);

		// Dark background.
		ctx.save();
		ctx.fillStyle = "rgba(0, 0, 0, 0.9)";
		ctx.fillRect(0, 0, canvas.width, canvas.height);
		ctx.restore();

		// Play area shadow.
		ctx.save();
		ctx.shadowColor = "black";
		ctx.shadowBlur = 20;
		ctx.fillRect(
			xStart,
			yStart,
			cellsXNum * cellSize,
			cellsYNum * cellSize
		);
		ctx.clearRect(
			xStart,
			yStart,
			cellsXNum * cellSize,
			cellsYNum * cellSize
		);
		ctx.restore();
		ctx.save();
		ctx.fillStyle = "rgba(255, 255, 255, 0.9)";
		ctx.fillRect(
			xStart,
			yStart,
			cellsXNum * cellSize,
			cellsYNum * cellSize
		);
		ctx.restore();

		// Draw exit cross
		let crossSize = cellSize > 24 ? cellSize : 24;
		crossSize = crossSize * 1.5;
		ctx.save();
		ctx.lineCap = "square";
		ctx.lineWidth = 4;
		ctx.strokeStyle = "white";
		ctx.fillStyle = "white";
		//ctx.shadowColor = "white"; // Shadow for cross and text
		//ctx.shadowBlur = 2;
		ctx.beginPath();
		const edge = 3;
		ctx.moveTo(window.innerWidth - crossSize, edge);
		ctx.lineTo(window.innerWidth - edge, crossSize);
		ctx.moveTo(window.innerWidth - edge, edge);
		ctx.lineTo(window.innerWidth - crossSize, crossSize);
		ctx.stroke();
		ctx.font = " " + Math.floor(crossSize / 2.5) + "px Lucida Sans Unicode";
		ctx.fillText(
			"close",
			window.innerWidth - crossSize,
			crossSize + Math.floor(crossSize / 2.5)
		);
		ctx.restore();

		if (showSplash) {
			drawSplash();
		} else {
			// Draw sys.
			ctx.save();
			let i, x, y;
			for (i = 0; i < sys.length; i++) {
				x = xStart + sys[i].x * cellSize;
				y = yStart + sys[i].y * cellSize;
				if (i == 0) ctx.fillStyle = "blue";
				else if (i == 1) ctx.fillStyle = "darkblue";
				else if (i == 2) ctx.fillStyle = "black";
				drawRoundRect(
					x + 1,
					y + 1,
					cellSize - 1,
					Math.floor(cellSize / 4)
				);
			}

			// Draw square.
			x = xStart + square.x * cellSize;
			y = yStart + square.y * cellSize;
			ctx.fillStyle = "red";
			//ctx.shadowColor = "yellow";
			//ctx.shadowBlur = 3;
			//let grd=ctx.createRadialGradient(75,50,5,90,60,100);
			//grd.addColorStop(0,"red");
			//grd.addColorStop(1,"black");
			//ctx.fillStyle=grd;
			drawRoundRect(x + 1, y + 1, cellSize - 1, Math.floor(cellSize / 4));
			ctx.restore();
		}

		if (sysEnd) {
			sysOver();
		}

		if (showGrid) drawGrid(cellSize, xStart, yStart);
		if (advanceSysState == null || advanceSysState == true)
			drawInterval = window.setTimeout(draw, frameTime, true);
	}

	function drawGrid(cellSize, xStart, yStart) {
		let i;
		for (i = 0; i <= cellsYNum; i++) {
			ctx.beginPath();
			ctx.moveTo(xStart, yStart + 0.5 + cellSize * i);
			ctx.lineTo(window.innerWidth - xStart, yStart + 0.5 + cellSize * i);
			ctx.stroke();
		}
		for (i = 0; i <= cellsXNum; i++) {
			ctx.beginPath();
			ctx.moveTo(xStart + 0.5 + cellSize * i, yStart);
			ctx.lineTo(
				xStart + 0.5 + cellSize * i,
				window.innerHeight - yStart
			);
			ctx.stroke();
		}
		ctx.save();
		ctx.font = cellSize + "px Monospace";
		ctx.fillStyle = "green";
		ctx.fillText("frameTime     : " + frameTime, xStart, yStart + cellSize);
		ctx.fillText(
			"frameTimeStart: " + frameTimeStart,
			xStart,
			yStart + cellSize * 2
		);
		ctx.fillText(
			"frameTimeMin  : " + frameTimeMin,
			xStart,
			yStart + cellSize * 3
		);
		ctx.fillText(
			"sysDirection  : " + sysDirection,
			xStart,
			yStart + cellSize * 4
		);
		ctx.fillText(
			"HeadX         : " + sys[0].x,
			xStart,
			yStart + cellSize * 5
		);
		ctx.fillText(
			"HeadY         : " + sys[0].y,
			xStart,
			yStart + cellSize * 6
		);
		ctx.fillText(
			"SquareX       : " + square.x,
			xStart,
			yStart + cellSize * 7
		);
		ctx.fillText(
			"SquareY       : " + square.y,
			xStart,
			yStart + cellSize * 8
		);
		ctx.restore();
	}

	return {
		createSystem: function () {
			createSystemCanvas();
		},
	};
})();
