use bevy::prelude::*;
use kayak_ui::prelude::{widgets::*, *};

#[derive(Component, Default, Clone, PartialEq)]
pub struct PrimaryButtonProps {
    /// The string to display
    pub content: String,
}

// In the future this will tell Kayak that these Props belongs to a widget.
// For now it's use to get the `WidgetName` component.
impl Widget for PrimaryButtonProps {}

// Now we need a widget bundle this can represent a collection of components our widget might have
// Note: You can include custom data here. Just don't expect it to get diffed during update!
#[derive(Bundle)]
pub struct PrimaryButtonBundle {
    pub props: PrimaryButtonProps,
    pub styles: KStyle,
    pub computed_styles: ComputedStyles,
    pub children: KChildren,
    pub on_event: OnEvent,
    pub widget_name: WidgetName,
}

impl Default for PrimaryButtonBundle {
    fn default() -> Self {
        Self {
            props: PrimaryButtonProps::default(),
            styles: KStyle::default(),
            computed_styles: ComputedStyles::default(),
            children: KChildren::default(),
            on_event: OnEvent::default(),
            widget_name: PrimaryButtonProps::default().get_name(),
        }
    }
}

pub fn primary_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    query: Query<&PrimaryButtonProps>,
) -> bool {
    if let Ok(props) = query.get(entity) {
        let parent_id = Some(entity);
        let styles = KStyle {
            background_color: StyleProp::Value(Color::hex("3523FE").unwrap()),
            border_radius: Corner::all(4.0).into(),
            border_color: StyleProp::Value(Color::hex("000000").unwrap()),
            color: StyleProp::Value(Color::hex("ffffff").unwrap()),
            height: StyleProp::Value(Units::Pixels(32.0)),
            padding: StyleProp::Value(Edge {
                bottom: Units::Pixels(8.0),
                top: Units::Pixels(8.0),
                right: Units::Pixels(0.0),
                left: Units::Pixels(0.0),
            }),
            ..Default::default()
        };

        rsx! {
            <BackgroundBundle
                styles={styles}
            >
                <TextWidgetBundle
                    text={TextProps {
                        content: props.content.clone(),
                        size: 14.0,
                        alignment: Alignment::Middle,
                        ..Default::default()
                    }}
                />
            </BackgroundBundle>
        };
    }
    true
}
