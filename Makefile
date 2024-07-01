release:
	cargo build --release

install:
	install -Dm755 target/release/framework-buddy /usr/bin/fish.june.framework-buddy
	install -Dm644 fish.june.pkexec.framework-buddy.policy /usr/share/polkit-1/actions/fish.june.pkexec.framework-buddy.policy
	install -Dm644 fish.june.framework-buddy.desktop /usr/share/applications/fish.june.framework-buddy.desktop

uninstall:
	rm /usr/bin/fish.june.framework-buddy
	rm /usr/share/polkit-1/actions/fish.june.pkexec.framework-buddy.policy
	rm /usr/share/applications/fish.june.framework-buddy.desktop
