pub fn diff<T: PartialEq + Clone>(new: &T, old: &T) -> Option<T> {
    if old == new {
        None
    } else {
        Some(new.clone())
    }
}

#[derive(Debug)]
pub enum NodeChange<Node, NodeDiff> {
    Swapped(Node, Node),
    Changed(NodeDiff),
    Identical,
}

impl <N, D> NodeChange<N,D> {
    pub fn is_identical(&self) -> bool{
        if let NodeChange::Identical = self {
            true
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub enum ChildChange<Node, NodeDiff> {
    Removed(usize, Node),
    Inserted(usize, Node),
    Changed(usize, NodeDiff),
}

pub trait Diffable where Self: Sized + Clone, {
    type Diff;

    fn diff(&self, old: Self) -> NodeChange<Self, Self::Diff>;
}

pub trait ToPlain<T> {
    fn to_plain(&self) -> T;
}

pub fn compare_render_children<Request : ToRender<Render, Node, Node::Diff>, Render : ToPlain<Node>, Node: Diffable>(
    mut new_children: Vec<Request>,
    mut old_children: Vec<Render>,
    compatible: bool
) -> (Vec<Render>, Vec<ChildChange<Node, Node::Diff>>){
    let common_len = std::cmp::min(new_children.len(), old_children.len());

    let added = new_children.split_off(common_len);
    let removed = old_children.split_off(common_len);
    let common: Vec<_> = old_children.into_iter().zip(new_children).collect();

    let (mut added_changes, mut added_render) : (Vec<_>, Vec<_>) = added.into_iter().enumerate().map(|(i, request)|{
        let render = request.render_new();
        (ChildChange::Inserted(i + common_len, render.to_plain()), render)
    }).unzip();

    let mut removed_changes : Vec<_> = removed.into_iter().enumerate().map(|(i, render)|{
        ChildChange::Removed(i+common_len, render.to_plain())
    }).collect();

    let (mut renders, nested_swapped_changes) : (Vec<_>, Vec<_>) = common.into_iter().enumerate().map(|(i, (render, request))|{
        let RenderResult{ render, diff } = request.render_update(render, compatible);
        let change = match diff {
            NodeChange::Identical => vec![],
            NodeChange::Swapped(new_swap, old_swap) => {
                vec![ChildChange::Removed(i, old_swap), ChildChange::Inserted(i, new_swap)]
            }
            NodeChange::Changed(diff) => vec![ChildChange::Changed(i, diff)],
        };
        (render, change)
    }).unzip();

    let mut changes: Vec<_> = nested_swapped_changes.into_iter().flatten().collect();
    changes.append(&mut added_changes);
    changes.append(&mut removed_changes);

    renders.append(&mut added_render);

    (renders, changes)
}

pub fn compare_children<Node: Diffable>(
    old: Vec<Node>,
    new: &Vec<Node>,
) -> Vec<ChildChange<Node, Node::Diff>> {
    let old_len = new.len();
    let mut changes: Vec<ChildChange<Node, Node::Diff>> = old
        .into_iter()
        .zip(0..old_len)
        .map(|(old_node, i)| match new.get(i) {
            None => vec![ChildChange::Removed(i, old_node)],
            Some(new_node) => match new_node.diff(old_node) {
                NodeChange::Identical => vec![],
                NodeChange::Swapped(new_swap, old_swap) => {
                    vec![ChildChange::Removed(i, old_swap), ChildChange::Inserted(i, new_swap)]
                }
                NodeChange::Changed(diff) => vec![ChildChange::Changed(i, diff)],
            },
        }).flatten()
        .collect();

    let new_len = new.len();
    if new_len > old_len {
        let mut removals: Vec<ChildChange<Node, Node::Diff>> = new
            .iter()
            .zip(old_len..(new_len - old_len))
            .skip(old_len)
            .map(|(new_one, i)| ChildChange::Inserted(i, (*new_one).clone()))
            .collect();
        changes.append(&mut removals);
    }
    changes
}

use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::ops::Range;

pub trait Renderer<Props, State, Request> {
    fn render(&self, props: &Props, state: &State) -> Request;
}

#[derive(Debug)]
pub struct ComponentImpl<Comp, Props, State, Request, Render, Node, Diff> {
    pub renderer: Comp,
    pub props: Props,
    pub state: State,
    pub request_type: PhantomData<Request>,
    pub render_type: PhantomData<Render>,
    pub processs_type: PhantomData<Node>,
    pub diff_type: PhantomData<Diff>,
}
#[derive(Debug)]
pub struct RequestedComponentImpl<Comp, Props, State, Request, Render, Node, Diff> {
    pub renderer: Comp,
    pub props: Props,
    pub state_type: PhantomData<State>,
    pub request_type: PhantomData<Request>,
    pub render_type: PhantomData<Render>,
    pub node_type: PhantomData<Node>,
    pub diff_type: PhantomData<Diff>,
}

#[derive(Debug)]
pub struct RenderedComponent<Render, Node, Diff> {
    pub renderer: Box<Component<Render, Node, Diff>>,
    pub rendered: Render,
}
pub struct UpdateResult<Render, Node, Diff> {
    pub rendered_component: RenderedComponent<Render, Node, Diff>,
    pub diff: NodeChange<Node, Diff>,
}
pub struct RenderResult<Render, Node, Diff> {
    pub render: Render,
    pub diff: NodeChange<Node, Diff>,
}
pub enum PreviousRender<Render, Node, Diff> {
    OfComponent(RenderedComponent<Render, Node, Diff>),
    OfRender(Render),
}

pub trait RequestedComponent<Render, Node, Diff>: Debug + Any {
    fn as_any(&mut self) -> &mut Any;
    fn render_new(self) -> RenderedComponent<Render, Node, Diff>;
    fn render_new_box(self : Box<Self>) -> RenderedComponent<Render, Node, Diff>;
    fn render_update(self, previous_render: PreviousRender<Render, Node, Diff>) -> UpdateResult<Render, Node, Diff>;
    fn render_update_box(self : Box<Self>, previous_render: PreviousRender<Render, Node, Diff>) -> UpdateResult<Render, Node, Diff>;
}
pub trait Component<Render: 'static, Node: 'static, Diff: 'static>: Debug + Any {
    fn as_any(&mut self) -> &mut Any;
    fn render_new(&self) -> Render;
    fn render_update(&self, old: Render, compatible: bool) -> RenderResult<Render, Node, Diff>;
}

/// To be implemented on the Request type
pub trait ToRender<Render, Node, Diff> {
    fn render_new(self) -> Render;
    fn render_update(self, old: Render, compatible: bool) -> RenderResult<Render, Node, Diff>;
}

impl<
        Comp: Debug + Renderer<Props, State, Request> + 'static,
        Props: PartialEq + Debug + 'static,
        State: Default + Debug + 'static,
        Request: ToRender<Render, Node, Diff> + Debug + 'static,
        Render: Debug + 'static,
        Node: Debug + 'static,
        Diff: Debug + 'static,
    > RequestedComponent<Render, Node, Diff>
    for RequestedComponentImpl<Comp, Props, State, Request, Render, Node, Diff>
{
    fn as_any(&mut self) -> &mut Any {
        self
    }

    fn render_new(
        self
    ) -> RenderedComponent<Render, Node, Diff> {
        let renderer: ComponentImpl<_, _, _, Request, _, _, _> = ComponentImpl {
            renderer: self.renderer,
            props: self.props,
            state: State::default(),
            request_type: PhantomData,
            render_type: PhantomData,
            processs_type: PhantomData,
            diff_type: PhantomData,
        };
        let rendered = renderer.render_new();
        RenderedComponent { renderer: Box::new(renderer), rendered }
    }

    fn render_new_box(self: Box<Self>) -> RenderedComponent<Render, Node, Diff> {
        self.render_new()
    }


    fn render_update(
        self,
        previous_render: PreviousRender<Render, Node, Diff>,
    ) -> UpdateResult<Render, Node, Diff> {
        match previous_render {
            PreviousRender::OfComponent(previous) => {
                let RenderedComponent {
                    mut renderer,
                    rendered,
                } = previous;
                let (maybe_new_renderer, rendered, diff) = {
                    let borrowed = (renderer).as_any().downcast_mut::<ComponentImpl<
                        Comp,
                        Props,
                        State,
                        Request,
                        Render,
                        Node,
                        Diff,
                    >>();
                    if let Some(same_component) = borrowed {
                        let (rendered, diff) = if same_component.props == self.props {
                            (rendered, NodeChange::Identical)
                        } else {
                            same_component.props = self.props;
                            unimplemented!();
                        };
                        drop(same_component);
                        (None, rendered, diff)
                    } else {
                        let new_renderer = ComponentImpl {
                            renderer: self.renderer,
                            props: self.props,
                            state: State::default(),
                            request_type: PhantomData,
                            render_type: PhantomData,
                            processs_type: PhantomData,
                            diff_type: PhantomData,
                        };
                        let RenderResult { render, diff } =
                            new_renderer.render_update(rendered, true);
                        (Some(new_renderer), render, diff)
                    }
                };
                let maybe_new_renderer: Option<
                    ComponentImpl<Comp, Props, State, Request, Render, Node, Diff>,
                > = maybe_new_renderer;
                let new_renderer = if let Some(def_new_renderer) = maybe_new_renderer {
                    let res: Box<Component<Render, Node, Diff>> = Box::new(def_new_renderer);
                    res
                } else {
                    renderer
                };
                UpdateResult {
                    rendered_component: RenderedComponent {
                        renderer: new_renderer,
                        rendered,
                    },
                    diff,
                }
            }
            PreviousRender::OfRender(previous_tree) => {
                let new_renderer: ComponentImpl<_, _, _, Request, _, _, _> = ComponentImpl {
                    renderer: self.renderer,
                    props: self.props,
                    state: State::default(),
                    request_type: PhantomData,
                    render_type: PhantomData,
                    processs_type: PhantomData,
                    diff_type: PhantomData,
                };
                let RenderResult { render, diff } =
                    new_renderer.render_update(previous_tree, false);
                UpdateResult {
                    rendered_component: RenderedComponent {
                        renderer: Box::new(new_renderer),
                        rendered: render,
                    },
                    diff,
                }
            },
        }
    }

    fn render_update_box(self: Box<Self>, previous_render: PreviousRender<Render, Node, Diff>) -> UpdateResult<Render, Node, Diff> {
        self.render_update(previous_render)
    }
}

impl<
        Comp: Renderer<Props, State, Request> + Debug + 'static,
        Props: PartialEq + Debug + 'static,
        State: Default + Debug + 'static,
        Request: ToRender<Render, Node, Diff> + Debug + 'static,
        Render: Debug + 'static,
        Node: Debug + 'static,
        Diff: Debug + 'static,
    > Component<Render, Node, Diff> for ComponentImpl<Comp, Props, State, Request, Render, Node, Diff>
{
    fn as_any(&mut self) -> &mut Any {
        self
    }

    fn render_new(&self) -> Render {
        let ComponentImpl { props, state, .. } = self;
        let new_request: Request = self.renderer.render(props, state);
        new_request.render_new()
    }

    fn render_update(&self, old: Render, compatible: bool) -> RenderResult<Render, Node, Diff> {
        let ComponentImpl { props, state, .. } = self;
        let new_request: Request = self.renderer.render(props, state);
        new_request.render_update(old, compatible)
    }
}
