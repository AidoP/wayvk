#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>

#include <libudev.h>
#include <libinput.h>

#include "vk.h"
#include "wl.h"
#include "session.h"
#include "term.h"

static int open_file(const char* path, int flags, void* user_data) {
	return open(path, flags);
}

static void close_file(int fd, void* user_data) {
	close(fd);
}

static const struct libinput_interface input_callbacks = {
	.open_restricted = open_file,
	.close_restricted = close_file
};


#define MODKEY MODIFIER_CMD
enum modifiers {
	MODIFIER_LCTRL = 0b10,
	MODIFIER_RCTRL = 0b100,
	MODIFIER_LALT = 0b1000,
	MODIFIER_RALT = 0b10000,
	MODIFIER_SHIFT = 0b100000,
	MODIFIER_ESC = 0b1000000,
	MODIFIER_CMD = 0b10000000
};

enum keys {
	KEY_ESC = 1,
	KEY_Q = 16,
	KEY_LCTRL = 29,
	KEY_SHIFT = 42,
	KEY_LALT = 56,

	KEY_F1 = 59,
	KEY_F2 = 60,
	KEY_F3 = 61,
	KEY_F4 = 62,
	KEY_F5 = 63,
	KEY_F6 = 64,
	KEY_F7 = 65,
	KEY_F8 = 66,
	KEY_F9 = 67,
	KEY_F10 = 68,

	KEY_RCTRL = 97,
	KEY_RALT = 100,
	KEY_CMD = 125,
};

int main(void) {
	srand(17);

	Vulkan vk = vk_setup();
	Wayland wl = wl_setup();
	struct udev* udev = udev_new();
	struct libinput* li = libinput_udev_create_context(&input_callbacks, NULL, udev);
	libinput_udev_assign_seat(li, "seat0");
	struct libinput_event* li_event;

	uint_fast8_t key_modifiers = 0;

	SessionHandler sessions[] = {
		term_session_handler,
		term_session_handler,
		term_session_handler,
	};
	#define sessions_len (sizeof(sessions) / sizeof(SessionHandler))
	uint_fast8_t active_session = 0;

	// Initialise all the sessions
	for (size_t index = 0; index < sessions_len; index++)
		session_setup(&vk ,&sessions[index]);

	//wl_display_run(wl.display);

	bool running = true;
	while (running) {
		libinput_dispatch(li);
		while ((li_event = libinput_get_event(li))) {
			switch (libinput_event_get_type(li_event)) {
			case LIBINPUT_EVENT_KEYBOARD_KEY:{
				struct libinput_event_keyboard* li_key_event = libinput_event_get_keyboard_event(li_event);
				uint32_t key_code = libinput_event_keyboard_get_key(li_key_event);
				enum libinput_key_state key_state = libinput_event_keyboard_get_key_state(li_key_event);
				printf("Got keycode %i\n", key_code);

				uint_least8_t modifier_bitmask = 0;
				switch (key_code) {
					case KEY_LCTRL:
						modifier_bitmask = MODIFIER_LCTRL;
						break;
					case KEY_RCTRL:
						modifier_bitmask = MODIFIER_RCTRL;
						break;
					case KEY_LALT:
						modifier_bitmask = MODIFIER_LALT;
						break;
					case KEY_RALT:
						modifier_bitmask = MODIFIER_RALT;
						break;
					case KEY_SHIFT:
						modifier_bitmask = MODIFIER_SHIFT;
						break;
					case KEY_ESC:
						modifier_bitmask = MODIFIER_ESC;
						break;
					case KEY_CMD:
						modifier_bitmask = MODIFIER_CMD;
						break;
					default:
						break;
				}
				
				if (modifier_bitmask) {
					// TODO - Handle multi-device cases
					if (key_state == LIBINPUT_KEY_STATE_PRESSED)
						key_modifiers |= modifier_bitmask;
					else if (key_state == LIBINPUT_KEY_STATE_RELEASED)
						key_modifiers &= !modifier_bitmask;
					break;
				}

				// Key press events
				if (key_state == LIBINPUT_KEY_STATE_PRESSED) {
					switch (key_code) {
						case KEY_Q:
							if (key_modifiers == (MODIFIER_SHIFT | MODKEY)) {
								running = false;
							}
							break;
						case KEY_F1:
						case KEY_F2:
						case KEY_F3:
						case KEY_F4:
						case KEY_F5:
						case KEY_F6:
						case KEY_F7:
						case KEY_F8:
						case KEY_F9:
						case KEY_F10:
							if (key_modifiers == MODKEY) {
								printf("Switching to %i out of %li\n", key_code - KEY_F1, sessions_len);
								uint_fast8_t session = key_code - KEY_F1;
								if (session < sessions_len)
									active_session = session;
							}
						default:
							break;
					}
				}

				break;
			}
			default:
				break;
			}
			libinput_event_destroy(li_event);
		}
		if (wl_event_loop_dispatch(wl.event_loop, 0))
			running = false;
		wl_display_flush_clients(wl.display);

		// Update the active session
		session_update(&vk, &sessions[active_session]);
	}


	// Clean up all the sessions
	for (size_t index = 0; index < sessions_len; index++)
		session_cleanup(&vk, &sessions[index]);
	wl_cleanup(&wl);
	vk_cleanup(&vk);

	libinput_unref(li);

	return 0;
}
