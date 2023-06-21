use lemmy_api_common::lemmy_db_views::structs::CommentView;
use relm4::prelude::*;
use gtk::prelude::*;
use relm4_components::web_image::WebImage;

use crate::api;
use crate::util::get_web_image_url;
use crate::util::markdown_to_pango_markup;
use crate::settings;

use super::voting_row::VotingRowModel;
use super::voting_row::VotingStats;

#[derive(Debug)]
pub struct CommentRow {
    comment: CommentView,
    avatar: Controller<WebImage>,
    voting_row: Controller<VotingRowModel>
}

#[derive(Debug)]
pub enum CommentRowMsg {
    OpenPerson,
    DeleteComment,
}

#[relm4::factory(pub)]
impl FactoryComponent for CommentRow {
    type Init = CommentView;
    type Input = CommentRowMsg;
    type Output = crate::AppMsg;
    type CommandOutput = ();
    type Widgets = PostViewWidgets;
    type ParentInput = crate::AppMsg;
    type ParentWidget = gtk::Box;

    view! {
        root = gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 10,
            set_margin_end: 10,
            set_margin_start: 10,
            set_margin_top: 10,

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 10,
                set_vexpand: false,

                if self.comment.creator.avatar.is_some() {
                    gtk::Box {
                        set_hexpand: false,
                        #[local_ref]
                        community_image -> gtk::Box {}
                    }
                } else {
                    gtk::Box {}
                },

                gtk::Button {
                    set_label: &self.comment.creator.name,
                    connect_clicked => CommentRowMsg::OpenPerson,
                },
            },
            
            gtk::Label {
               set_markup: &markdown_to_pango_markup(self.comment.comment.content.clone()),
               set_halign: gtk::Align::Start,
               set_use_markup: true,
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                #[local_ref]
                voting_row -> gtk::Box {},

                if self.comment.creator.id.0 == settings::get_current_account().id {
                    gtk::Button {
                        set_icon_name: "edit-delete",
                        connect_clicked => CommentRowMsg::DeleteComment,
                        set_margin_start: 10,
                    }
                } else {
                    gtk::Box {}
                }
            },
            
            gtk::Separator {}
        }
    }

    fn forward_to_parent(output: Self::Output) -> Option<Self::ParentInput> {
        Some(output)
    }

    fn init_model(value: Self::Init, _index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        let avatar = WebImage::builder().launch(get_web_image_url(value.community.clone().icon)).detach();
        let voting_row = VotingRowModel::builder().launch(VotingStats::from_comment(value.clone().counts, value.my_vote)).detach();

        Self { comment: value, avatar, voting_row }
    }

    fn init_widgets(
            &mut self,
            _index: &Self::Index,
            root: &Self::Root,
            _returned_widget: &<Self::ParentWidget as relm4::factory::FactoryView>::ReturnedWidget,
            sender: FactorySender<Self>,
        ) -> Self::Widgets {
        let community_image = self.avatar.widget();
        let voting_row = self.voting_row.widget();
        let widgets = view_output!();
        widgets
    }

    fn update(&mut self, message: Self::Input, sender: FactorySender<Self>) {
        match message {
            CommentRowMsg::OpenPerson => {
                sender.output(crate::AppMsg::OpenPerson(self.comment.creator.name.clone()))
            }
            CommentRowMsg::DeleteComment => {
                let comment_id = self.comment.comment.id;
                std::thread::spawn(move || {
                    let _ = api::comment::delete_comment(comment_id);
                    let _ = sender.output(crate::AppMsg::StartFetchPosts(None));
                });
            }
        }
    }
}