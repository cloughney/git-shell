#!/usr/local/bin/node

var fs = require("fs"),
	newline = require("os").EOL,
	FileLogger = function() {
		function e() {
			var e = this;
			this.debug = function(o) {
				e.write("DEBUG", o)
			}, this.warn = function(o) {
				e.write("WARN", o)
			}, this.error = function(o) {
				e.write("ERROR", o)
			}, this.write = function(e, o) {
				var t = new Date,
					r = t.getHours() + ":" + t.getMinutes() + ":" + t.getSeconds();
				o = [r, e, o + newline].join("   "), fs.appendFileSync("gitshell.log", o)
			}
		}
		return e
	}(),
	InputValidator = function() {
		function e(e) {
			this.log = e
		}
		return e.prototype.validateInput = function() {
			(!process.argv.length || process.argv.length < 3) && process.exit();
			var e = {
					isValid: !1,
					user: null,
					command: null,
					repository: null
				},
				o = process.env.SSH_ORIGINAL_COMMAND;
			if (process.argv.length < 3 || !(e.user = process.argv[2])) return console.log("warn"), this.log.warn("Script executed with no user argument."), e;
			if (!o) return this.log.debug("SSH_ORIGINAL_COMMAND is empty."), e;
			var t = o.split(" ");
			return 2 !== t.length ? e : (e.command = t[0], e.repository = t[1], this.validateCommand(e) && this.validateRepository(e) ? (e.isValid = !0, e) : e)
		}, e.prototype.validateCommand = function(e) {
			var o = /^git-upload-pack|git-receive-pack|git-upload-archive$/;
			return e.command && o.test(e.command) ? !0 : !1
		}, e.prototype.validateRepository = function(e) {
			var o = /^([a-zA-Z0-9\.\-_]+)|('[a-zA-Z0-9\.\-_]+')$/;
			return o.test(e.repository) ? (0 === e.repository.indexOf("'") && (e.repository = e.repository.substr(1, e.repository.length - 2)), !0) : !1
		}, e
	}(),
	exec = require("child_process").execSync;
! function() {
	var e = new FileLogger,
		o = new InputValidator(e),
		t = o.validateInput();
	e.debug("Input: { user: " + t.user + ", command: " + t.command + ", repository: " + t.repository + " }"), t.user || process.exit(), t.isValid || (console.log("You are authenticated as '" + t.user + "', but you do not have shell access."), process.exit());
	try {
		var r = exec(t.command + " '" + t.repository + "'");
		e.debug(r), console.log(r)
	} catch (n) {
		e.error(n.message)
	}
}();
var ConsoleLogger = function() {
	function e() {}
	return e.prototype.debug = function(e) {
		console.log("DEBUG : " + e)
	}, e.prototype.warn = function(e) {
		console.log("WARN : " + e)
	}, e.prototype.error = function(e) {
		console.log("ERROR : " + e)
	}, e
}();