import { useTranslation } from 'react-i18next'
import { TitleBar } from '@/components/title-bar'
import { Tabs, TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs'

export default function App() {
  const { t } = useTranslation()

  return (
    <div className="flex flex-col h-screen bg-background/80 backdrop-blur-md font-sans">
      <TitleBar />
      <main className="flex-1 overflow-auto flex items-start justify-center p-8">
        <div className="max-w-md w-full space-y-4">
          <h1 className="text-2xl font-bold text-foreground">{t('settings.title')}</h1>
          <Tabs defaultValue="theme">
            <TabsList className="grid w-full grid-cols-2">
              <TabsTrigger value="theme">{t('settings.tabs.theme')}</TabsTrigger>
              <TabsTrigger value="language">{t('settings.tabs.language')}</TabsTrigger>
            </TabsList>
            <TabsContent value="theme" className="text-sm text-muted-foreground py-4">
              {t('settings.theme.placeholder')}
            </TabsContent>
            <TabsContent value="language" className="text-sm text-muted-foreground py-4">
              {t('settings.language.placeholder')}
            </TabsContent>
          </Tabs>
        </div>
      </main>
    </div>
  )
}
