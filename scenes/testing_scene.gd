extends Node

func _ready() -> void:
	CoreDialog.load_track("res://Dialogic/example.json")
	CoreDialog.event_bus.track_ended.connect(_on_track_end)

func _on_track_end() -> void:
	get_tree().quit()
