use iced::advanced::renderer::Quad;
use iced::advanced::widget::Tree;
use iced::advanced::{Layout, Widget, layout, renderer};
use iced::widget::{Button, Column, Row, container, grid, text};
use iced::{Color, Element, Length, Size, Vector, border};
use whoops_core::grid::Grid;
use whoops_core::grid::rewindable::Rewindable;
use whoops_core::grid::tile_grid::TileGrid;
use whoops_core::pos::Pos;
use whoops_core::tile::Tile;

const UNKNOWN_COLOR: Color = Color::from_rgb8(0x22, 0x24, 0x26);
const WALL_COLOR: Color = Color::from_rgb8(0xaa, 0x00, 0x10);
const DOT_COLOR: Color = Color::from_rgb8(0x16, 0x9a, 0xb3);
const BACKGROUND: Color = Color::from_rgb8(0x18, 0x1a, 0x1b);

pub struct Game {
    grid: Rewindable<TileGrid>,
}

#[derive(Clone)]
pub enum Message {
    TileMessage(Pos, TileMessage),
}

impl Game {
    pub fn new(grid: impl Into<Rewindable<TileGrid>>) -> Self {
        let grid = grid.into();
        Self { grid }
    }

    pub fn update(&mut self, message: Message) {}

    pub fn view(&self) -> Element<'_, Message> {
        let h = self.grid.height();
        let w = self.grid.width();

        let pos = Pos::new(0, 0);
        // Element::new(canvas(TileCanvas(self.grid.get(pos).unwrap())))
        //     .map(move |msg| Message::TileMessage(pos, msg))
        //     .explain(Color::WHITE)

        Element::new(GridWidget(&self.grid))

        // Element::new(
        //     Column::with_children((0..h).map(|y| {
        //         Row::with_children((0..w).map(move |x| {
        //             let pos = Pos::new(x, y);
        //             let tile = self.grid.get(pos).unwrap();
        //             let widget = TileWidget(tile);
        //             Element::new(widget)
        //         }))
        //         .width(Length::Fill)
        //         .height(Length::Fill)
        //         .into()
        //     }))
        //     .width(Length::Fill)
        //     .height(Length::Fill)
        // )
        // .explain(Color::WHITE)
    }
}

struct TileState<P>
where
    P: iced::advanced::text::Paragraph
{
    paragraph: iced::advanced::text::paragraph::Plain<P>,
}

struct TileWidget(pub Tile);

#[derive(Clone)]
pub enum TileMessage {
    Next,
    Prev,
}

impl<M, Theme, Renderer> Widget<M, Theme, Renderer> for TileWidget
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'static,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn state(&self) -> iced::advanced::widget::tree::State {
        iced::advanced::widget::tree::State::new(
            TileState::<Renderer::Paragraph> {
                paragraph: Default::default(),
            }
        )
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let Size { width, height } = limits.max();
        let size = width.min(height);

        let node_size = Size::new(size, size);

        let Some(value) = self.0.as_value() else {
            return layout::Node::new(node_size);
        };

        let font_size = size / 2.;
        let state = tree.state.downcast_mut::<TileState<Renderer::Paragraph>>();
        let content = format!("{value}");

        text::layout(
            &mut state.paragraph,
            renderer,
            &layout::Limits::new(node_size, node_size),
            &content,
            text::Format {
                size: Some(iced::Pixels(font_size)),
                // font: todo!(),
                align_x: text::Alignment::Center,
                align_y: iced::alignment::Vertical::Center,
                ..Default::default()
            }
        )
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        _style: &iced::advanced::renderer::Style,
        layout: Layout<'_>,
        _cursor: iced::advanced::mouse::Cursor,
        _viewport: &iced::Rectangle,
    ) {
        let color = match self.0 {
            Tile::Unknown => UNKNOWN_COLOR,
            Tile::Wall => WALL_COLOR,
            Tile::Dot(_) => DOT_COLOR,
        };

        // TODO: draw text

        let bounds = layout.bounds();
        let radius = bounds.width;
        let half_size = bounds.width / 2.;
        let padding = half_size * 0.05;
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds().shrink(padding),
                border: border::rounded(radius),
                ..Default::default()
            },
            color,
        );
    }
}

struct GridWidget<'a>(pub &'a dyn Grid);

impl<M, Theme, Renderer> Widget<M, Theme, Renderer> for GridWidget<'_>
where
    Renderer: iced::advanced::Renderer + iced::advanced::text::Renderer + 'static,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let Size { width, height } = limits.max();
        let tile_w = width / self.0.width() as f32;
        let tile_h = height / self.0.height() as f32;
        let tile_size = tile_w.min(tile_h);
        let node_size = Size::new(tile_size, tile_size);

        let children = (0..self.0.height())
            .flat_map(|y| {
                (0..self.0.width()).map(move |x| {
                    let translation = Vector::new(x as f32, y as f32) * tile_size;
                    layout::Node::new(node_size).translate(translation)
                })
            })
            .collect();

        let grid_size = Size::new(
            tile_size * self.0.width() as f32,
            tile_size * self.0.height() as f32,
        );

        let offset = Vector::new(tile_w - tile_size, tile_h - tile_size) / 2.;
        layout::Node::with_children(grid_size, children).translate(offset)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                ..Default::default()
            },
            BACKGROUND,
        );

        for (i, layout) in layout.children().enumerate() {
            let w = self.0.width();
            let x = i as u32 % w;
            let y = i as u32 / w;
            let pos = Pos::new(x, y);
            let tile = self.0.get(pos).unwrap();
            let tile_widget = TileWidget(tile);
            <TileWidget as Widget<M, Theme, Renderer>>::draw(
                &tile_widget,
                tree,
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            )
        }
    }
}
