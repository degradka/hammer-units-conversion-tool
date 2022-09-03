extends Panel

# Metric
var meter = 1
var decimeter = 0.1
var centimeter = 0.01

# Imperial
var yard = 0.9144
var foot = 0.3048
var inch = 0.0254

# Hammer
var skybox_unit = 0.3048
var unit = 0.01905

# Left side
var value_left: float
var input_left = 0
var index_left: int
func _on_OptionButton_item_selected(index):
	index_left = index
	update_value_right()
	update_value_left()

# Right side
var value_right: float
var input_right = 0
var index_right: int
func _on_OptionButton2_item_selected(index):
	index_right = index
	update_value_left()
	update_value_right()

onready var result = get_node("Result")
onready var input = get_node("UserInput")

func _ready() -> void:
	update_value_right()
	update_value_left()

func _on_LineEdit_text_changed(new_text):
	input_left = new_text
	update_value_right()

func _on_Result_text_changed(new_text):
	input_right = new_text
	update_value_left()

func update_value_right():
	set_result_right(input_left)

func update_value_left():
	set_result_left(input_right)

func set_result_right(left_value):
	result.text = str(converter(get_current_type_left(index_left), get_current_type_right(index_right), float(left_value)))

func set_result_left(right_value):
	input.text = str(converter(get_current_type_right(index_right), get_current_type_left(index_left), float(right_value)))

func get_current_type_left(index):
	match index:
		0: return meter
		1: return decimeter
		2: return centimeter
		3: return yard
		4: return foot
		5: return inch

func get_current_type_right(index):
	match index:
		0: return unit
		1: return skybox_unit

func converter(from: float, to: float, values: float):
	return stepify((values * (from / to)), 0.01)



