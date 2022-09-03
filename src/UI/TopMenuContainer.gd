extends Panel

func _ready():
	$MenuItems/HelpMenu.get_popup().add_item("VHUC Github")
	#$MenuItems/HelpMenu.get_popup().add_item("About VHUC")
	
	$MenuItems/HelpMenu.get_popup().connect("id_pressed", self, "_on_item_pressed")

func _on_item_pressed(id):
	var item_name = $MenuItems/HelpMenu.get_popup().get_item_text(id)
	if item_name == "VHUC Github":
		OS.shell_open("https://github.com/degradka/Valve-Hammer-Units-Converter")
