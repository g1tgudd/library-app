use serde::{Serialize, Deserialize};
use yew::{prelude::*, services::ConsoleService};
use serde_json::{from_str, Value, from_value, to_string_pretty};
use yew::{
    format::{ Json, Nothing },
    services::{
        fetch::{FetchService, FetchTask, Request, Response},
    },
};

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UpdateRecord {
//     // pub index: String,
//     // pub document_id: String,
//     pub data: Value
// }

pub enum Msg {
    ToggleEditRecord,
    ValidateInputJson(String),
    RequestUpdateRecord,
    Ignore
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct WindowEditRecordProps {
    // #[prop_or(String::from("this is value"))]
    #[prop_or(false)]
    pub display_edit_record: bool,

    #[prop_or_default]
    pub edit_data: String,
    #[prop_or_default]
    pub edit_index: String, //card_id

    pub on_toggle_editrecord:Callback<Msg>,

    #[prop_or_default]
    pub app_id: String,
    #[prop_or_default]
    pub card_index: String,

    
}


pub struct EditRecord {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    props: WindowEditRecordProps,
    callback_toggle_editecord: Callback<Msg>,
    //Validasi Input / Edit Json
    value: String,
    json_is_valid: bool,

    textarea_string: String,

    fetch_task: Option<FetchTask>,
    request_success: bool,

    app_id: String,
    app_name: String,

    loading: bool,
}

impl Component for EditRecord {
    type Message = Msg;
    type Properties = WindowEditRecordProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {

        let textarea_value = props.edit_data.clone();
        let textarea_parse:Value = serde_json::from_str(&textarea_value).unwrap();
        let textarea_pretty = serde_json::to_string_pretty(&textarea_parse).unwrap();

        Self {
            link,
            callback_toggle_editecord: props.on_toggle_editrecord.clone(),

            app_id: props.app_id.clone(),
            app_name: props.card_index.clone(),

            props,
            value: "".to_string(),
            json_is_valid: false,

            textarea_string: textarea_pretty,

            fetch_task: None,
            request_success: false,

            loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleEditRecord => {
                // ConsoleService::info(&format!("DEBUG : self.delete_index MODAL COMP:{:?}", self.props.edit_index.clone()));
                // ConsoleService::info(&format!("DEBUG : self.card_index MODAL COMP:{:?}", self.props.card_index.clone()));
                self.callback_toggle_editecord.emit(Msg::ToggleEditRecord);
                true
            }
            Msg::ValidateInputJson (data) => {

                self.value = data;
                self.json_is_valid = match serde_json::from_str::<serde_json::Value>(&self.value) {
                    Ok(_) => true,
                    Err(_) => false,
                };
                // ConsoleService::info(&format!("DEBUG : value:{:?}", self.value));
                // ConsoleService::info(&format!("DEBUG : json_is_valid:{:?}", self.json_is_valid));
                true
            }

            Msg::RequestUpdateRecord => {
                self.loading = true;
                let mut records = serde_json::json!({});
                match serde_json::from_str::<serde_json::Value>(&self.value) {
                    Ok(create) => records = create,
                    Err(Error) => ConsoleService::info(&format!("Data Input = {}", &Error)),
                };

                let url = format!("http://localhost:1234/book/{}/{}/{}", &self.app_id, &self.props.card_index, &self.props.edit_index.replace("\"", ""));
                let request = Request::put(url)
                    .header("Content-Type", "application/json")
                    .body(Json(&records))
                    .expect("Could not build request.");
                
                let callback = 
                    self.link.callback(|response: Response<Json<Result<String, anyhow::Error>>>| {
                        let (meta, Json(data)) = response.into_parts();
        
                        match data { 
                            Ok(dataok) => {
                                // ConsoleService::info(&format!("data response {:?}", &dataok));
                                Msg::Ignore
                            }
                            Err(error) => {
                                Msg::Ignore
                            }
                        }
                    });
        
                let task = FetchService::fetch(request, callback).expect("failed to start request");
                
                self.fetch_task = Some(task);
                
                true
            }

            Msg::Ignore => {
                self.loading = false;
                self.request_success = true;
                true
            }
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            ConsoleService::info(&format!("data MODAL card_index / Index name {:?}", self.props.card_index));
            ConsoleService::info(&format!("data MODAL app_id {:?}", self.props.app_id));
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.edit_data != props.edit_data {
            self.props.edit_data = props.edit_data;
            // self.role_permissions = props.role_permissions;
            true
        }else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"EDIT BOOK DETAILS "}{self.props.edit_index.clone()}</h1>
                        
                        <button 
                            type="button" 
                            class="window-index-closebutton"
                            onclick=self.link.callback(|_| Msg::ToggleEditRecord)>
                                <img src="images/close.png" alt="close window" style="width: 32px"/> 
                        </button>
                    </div> 

                    <h6>{"Edit this book detail's JSON values"}
                     </h6>

                    <div class="window-submit-form">
                        <form class="record-text-input" id="submit-editrecord">
                            <textarea 
                                type="text" 
                                class="insert-record" 
                                style="font-size:12px;font-weight: bold; line-height: 1.4;"

                                oninput = self.link.callback(|data: InputData| Msg::ValidateInputJson(data.value))
                                >
                            {self.textarea_string.clone()}  
                            </textarea>
                        </form>   
                    </div> 

                    {
                        if self.json_is_valid {
                            if self.loading{
                                html!{
                                    <button 
                                    type="submit"
                                    form="submit-insertrecord"
                                    class="window-confirm-button"
                                    // onclick = self.link.callback(|_| Msg::RequestUpdateRecord)
                                    >
                                        <span class="loader">
                                            <span class="loader-inner">
                                            </span>
                                        </span>
                                    </button>
                                }
                            } else {
                                html!{
                                    <button 
                                    type="submit"
                                    form="submit-editrecord"
                                    class="window-confirm-button"
                                    onclick = self.link.callback(|_| Msg::RequestUpdateRecord)

                                    // onclick=self.link.batch_callback(|_| vec![
                                    //     Msg::RequestUpdateRecord,
                                    //     Msg::ToggleEditRecord,
                                    // ])
                                    >
                                    { "EDIT BOOK DETAIL" }
                                    </button>
                                }
                            }
                        } else {
                            html! {
                                <button disabled=true class="window-confirm-button">
                                    {"FORM INPUT MUST BE IN JSON FORMAT!"}
                                </button> 
                            }
                        }
                    }
                    
                </div>
                {
                    if self.request_success {
                        html!{
                            {self.modal_success()}
                        }
                            
                    } else {
                        html!{}
                    }
                }
            </div>
        }
    }
}
impl EditRecord {
    fn modal_success(&self) -> Html {
        html! {
            <div class="window-overlay">
                <div class="window-index" id="create-index"> 

                    <div class="top-row-index-window-insert">
                        <h1>{"EDIT RECORD SUCCESSFUL"}</h1>
                    </div> 

                    <button 
                        type="submit"
                        form="submit-deletecard"
                        class="window-confirm-button"
                        onclick=self.link.callback(|_| Msg::ToggleEditRecord)
                    >
                        { "OKAY" }
                    </button>  
                </div>
            </div>
        }
    }
}