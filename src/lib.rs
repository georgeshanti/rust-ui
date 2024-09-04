pub mod core {
    use std::any::Any;

    use widget_drive::Widget;

    pub struct Element{}

    type Renderer = fn(widget: &mut dyn Widget);

    pub trait Widget {
        fn add_renderer(self: &mut Self, renderer: Renderer);
        fn draw(self: &mut Self) -> Element;
        fn update_props(self: &mut Self, props: &dyn Any);
        fn render(self: &mut Self);
    }

    pub trait ProppedWidgetDefaults<Props>: Widget {
        fn get_props(self: &Self) -> Props;
    }

    pub trait ProppedWidget<Props>: Widget {
        fn create(props: Props) -> Self;
        fn build(self: &mut Self) -> &mut dyn Widget;
    }

    #[derive(Copy, Clone)]
    pub struct AppProps{}

    #[derive(Widget)]
    pub struct App {
        #[props] props: AppProps,
        renderer: Option<Renderer>,
    }

    impl<'a> ProppedWidget<AppProps> for App {

        fn create(props: AppProps) -> Self {
            App{
                props,
                renderer: None
            }
        }
    
        fn build(self: &mut Self) -> &mut dyn Widget {
            todo!()
        }
    }

    // fn d() {
    //     let a: &mut dyn Widget = &mut App::create(AppProps{});
    // }

}